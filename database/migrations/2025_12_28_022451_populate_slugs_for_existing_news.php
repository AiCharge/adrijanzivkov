<?php

use App\Models\News;
use Illuminate\Database\Migrations\Migration;
use Illuminate\Support\Str;

return new class extends Migration
{
    /**
     * Run the migrations.
     */
    public function up(): void
    {
        News::chunk(100, function ($newsItems) {
            foreach ($newsItems as $news) {
                $news->update([
                    'slug' => Str::slug($news->title),
                ]);
            }
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        // Cannot reverse slug population - slugs are required
    }
};
