<?php

namespace App\Console\Commands;

use Exception;
use Illuminate\Console\Command;
use Illuminate\Database\Connection;
use Illuminate\Database\QueryException;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Schema;

class MigrateDataToPostgres extends Command
{
    protected $signature = 'db:copy-to-postgres {--truncate : Empty each target table before inserting, so the command is safe to re-run}';

    protected $description = 'Copy all table data from the current database into a PostgreSQL connection entered at runtime';

    protected $pgConnectionName = 'dynamic_pg';

    public function handle(): int
    {
        $pg = $this->setupPgConnection();

        $tables = $this->getSourceTables();
        $tables = $this->filterExcludedTables($tables);

        if (empty($tables)) {
            $this->warn('No tables left to copy after applying exclude filter.');

            return 0;
        }

        if ($this->option('truncate')) {
            $this->truncateTargetTables($tables, $pg);
        }

        $this->copyTablesData($tables, $pg);

        $this->info('Data copy complete.');

        return 0;
    }

    private function setupPgConnection(): Connection
    {
        $host = $this->ask('host');
        $port = $this->ask('port', 5432);
        $database = $this->ask('database', 'main');
        $username = $this->ask('username', 'laravel');
        $password = $this->ask('password');

        config([
            "database.connections.{$this->pgConnectionName}" => [
                ...config('database.connections.pgsql'),
                'host' => $host,
                'port' => $port,
                'database' => $database,
                'username' => $username,
                'password' => $password,
            ],
        ]);

        $this->info("Postgres connection '{$this->pgConnectionName}' configured.");

        return DB::connection($this->pgConnectionName);
    }

    private function getDbName(): ?string
    {
        return config('database.connections.'.config('database.default').'.database');
    }

    private function getSourceTables(): array
    {
        $scheme = $this->getDbName();
        $tables = Arr::pluck(Schema::getTables($scheme), 'name');

        if (empty($tables)) {
            $this->error('No tables found in the current database.');
        } else {
            $this->info('Found tables: '.Arr::join($tables, ', '));
        }

        return $tables;
    }

    /**
     * @return string[]
     */
    private function filterExcludedTables(array $tables): array
    {
        $exclude = $this->ask('exclude');
        $excludedTables = $exclude ? array_map('trim', explode(',', $exclude)) : [];
        $excludedTables = [...$excludedTables, 'migrations', 'sessions', 'jobs', 'failed_jobs', 'job_batches', 'password_reset_tokens', 'cache', 'cache_locks'];

        $filtered = array_filter($tables, fn ($table) => ! in_array($table, $excludedTables));

        if (! empty($excludedTables)) {
            $this->line('Excluded tables: '.implode(', ', $excludedTables));
            $this->line('Tables to copy after exclusion: '.implode(', ', $filtered));
        }

        return $filtered;
    }

    /**
     * Empty every target table in a single statement before any inserts run.
     *
     * This cannot be folded into the copy loop: inserting requires parents before
     * children, while deleting requires the exact opposite, so no single ordering
     * satisfies both. TRUNCATE accepts every table at once and resolves the order
     * itself, and CASCADE covers any dependent table not named in the list.
     */
    private function truncateTargetTables(array $tables, Connection $pg): void
    {
        $existing = array_values(array_filter(
            $tables,
            fn ($table) => Schema::connection($this->pgConnectionName)->hasTable($table)
        ));

        if (empty($existing)) {
            return;
        }

        $quoted = implode(', ', array_map(fn ($table) => '"'.$table.'"', $existing));

        $pg->statement("TRUNCATE TABLE {$quoted} RESTART IDENTITY CASCADE");

        $this->warn('Emptied '.count($existing).' target tables: '.implode(', ', $existing));
    }

    private function copyTablesData(array $tables, Connection $pg): void
    {
        try {
            $left = collect($tables);

            // Guard against an unresolvable foreign key (e.g. an orphaned row whose
            // parent was hard-deleted). Without this the retry loop spins forever,
            // because such a table can never succeed no matter how often it is
            // deferred. If a full pass over the remaining tables makes no progress,
            // the ordering is not the problem and we stop with the real reason.
            $sinceProgress = 0;
            $blocked = [];

            while ($left->isNotEmpty()) {
                try {
                    $this->line('Tables left: '.($left->count()).' '.$left->join(', '));
                    $table = $left->shift();
                    $this->copyTableData($table, $pg);
                    $sinceProgress = 0;
                    unset($blocked[$table]);
                } catch (QueryException $e) {
                    if ($e->getCode() === '23503') { // foreign key violation
                        $left->push($table);
                        $blocked[$table] = str($e->getMessage())->words(40)->toString();
                        $this->error("Table '{$table}' has foreign key constraints. Retrying later.");
                        $this->warn($blocked[$table]);

                        if (++$sinceProgress > $left->count()) {
                            $this->newLine();
                            $this->error('Stopping: a full pass over the remaining tables copied nothing, so this is not an ordering problem.');
                            $this->error('These tables reference rows that do not exist in the target - most likely orphaned data in the source:');
                            foreach ($blocked as $t => $reason) {
                                $this->line("  - {$t}: {$reason}");
                            }

                            throw new Exception('Unresolvable foreign key violations in: '.implode(', ', array_keys($blocked)));
                        }
                    } else {
                        throw $e;
                    }
                } finally {
                    $this->line('-----------------------');
                }
            }
        } catch (Exception $e) {
            $this->error('Error during data copy: '.$e->getMessage());
            exit(1);
        }
    }

    /**
     * Point the table's identity sequence past the highest id that was just copied.
     *
     * Rows keep their original ids, so without this the sequence is still at its
     * starting value and the first insert after cutover collides with existing data.
     */
    private function resetPostgresSequence(string $table, Connection $pg): void
    {
        $primaryKey = $pg->selectOne('
            SELECT a.attname
            FROM pg_index i
            JOIN pg_attribute a ON a.attrelid = i.indrelid AND a.attnum = ANY(i.indkey)
            WHERE i.indrelid = ?::regclass AND i.indisprimary
        ', [$table])->attname ?? null;

        if (! $primaryKey) {
            return;
        }

        $sequence = $pg->selectOne('SELECT pg_get_serial_sequence(?, ?) AS seq', [$table, $primaryKey])->seq ?? null;

        if (! $sequence) {
            return;
        }

        $maxId = $pg->table($table)->max($primaryKey) ?: 0;

        $pg->statement('SELECT setval(?, ?, ?)', [
            $sequence,
            $maxId > 0 ? $maxId : 1,
            $maxId > 0,
        ]);
    }

    private function insert(string $table, Connection $pg, Collection $data, array $excluded): void
    {
        $count = $data->count();

        $data = $data->values()->map(fn ($item) => Arr::except((array) $item, $excluded))->all();

        if (empty($data)) {
            $this->warn("No rows to insert for '{$table}' after excluding columns.");

            return;
        }

        $chunks = array_chunk($data, 500);
        foreach ($chunks as $chunk) {
            $pg->table($table)->insert($chunk);
        }

        $this->info("Copied $count rows into '{$table}'.");
    }

    private function copyTableData(string $table, Connection $pg): void
    {
        $this->line("Copying table '{$table}' ...");

        if (! Schema::connection($this->pgConnectionName)->hasTable($table)) {
            $this->warn("Table '{$table}' does not exist in Postgres DB. Skipping.");

            return;
        }

        $generated = $this->getGeneratedColumns($table);
        if (count($generated)) {
            $this->warn('Found generated columns: '.implode(',', $generated));
        }

        $columns = Arr::pluck(Schema::getColumns($table), 'name');
        $this->line('Local  columns: '.implode(', ', $columns));

        $remoteColumns = Arr::pluck(Schema::connection($this->pgConnectionName)->getColumns($table), 'name');
        $this->line('Remote columns: '.implode(', ', $remoteColumns));

        $diff = array_diff($columns, $remoteColumns);

        if (! empty($diff)) {
            $this->error('Columns mismatch: '.implode(', ', $diff));

            $confirm = $this->confirm('Continue?', true);
            if (! $confirm) {
                exit(1);
            }
            $generated = [...$generated, ...$diff];
        }

        // One transaction per table, so a chunk failing part-way through does not
        // leave the table half-populated. That matters because this command does
        // not truncate by default: a partial table would otherwise turn every
        // re-run into a duplicate-key error on the rows that already landed.
        $pg->transaction(function () use ($table, $pg, $generated) {
            if (Schema::hasColumn($table, 'id')) {
                DB::table($table)->chunkById(1000, function ($chunk) use ($pg, $table, $generated) {
                    $this->insert($table, $pg, $chunk, $generated);
                });
            } else {
                $data = DB::table($table)->get();
                $this->insert($table, $pg, $data, $generated);
            }
        });

        $this->resetPostgresSequence($table, $pg);
    }

    /**
     * Generated columns must not be inserted explicitly. Only MySQL exposes them
     * through INFORMATION_SCHEMA.EXTRA, and MySQL is the source being migrated
     * away from, so any other source driver simply reports none.
     *
     * @return string[]
     */
    private function getGeneratedColumns(string $table): array
    {
        $driver = config('database.connections.'.config('database.default').'.driver');

        if (! in_array($driver, ['mysql', 'mariadb'])) {
            return [];
        }

        $dbName = $this->getDbName();

        $columns = DB::select("
            SELECT COLUMN_NAME
            FROM INFORMATION_SCHEMA.COLUMNS
            WHERE TABLE_SCHEMA = ?
              AND TABLE_NAME = ?
              AND EXTRA LIKE '%GENERATED%'
        ", [$dbName, $table]);

        return collect($columns)->pluck('COLUMN_NAME')->toArray();
    }
}
