<?php

namespace App\Filament\Resources\StorySubmissions\Tables;

use App\Enums\StorySubmissionStatus;
use Filament\Actions\EditAction;
use Filament\Tables\Columns\BadgeColumn;
use Filament\Tables\Columns\TextColumn;
use Filament\Tables\Filters\SelectFilter;
use Filament\Tables\Table;

class StorySubmissionsTable
{
    public static function configure(Table $table): Table
    {
        return $table
            ->columns([
                TextColumn::make('name')
                    ->label('Ime')
                    ->searchable()
                    ->sortable(),
                TextColumn::make('phone')
                    ->label('Телефон'),
                TextColumn::make('message')
                    ->label('Приказна')
                    ->limit(60)
                    ->tooltip(fn ($record) => $record->message),
                BadgeColumn::make('status')
                    ->label('Статус')
                    ->colors([
                        'info' => StorySubmissionStatus::New,
                        'success' => StorySubmissionStatus::Accepted,
                        'danger' => StorySubmissionStatus::Rejected,
                        'gray' => StorySubmissionStatus::Completed,
                    ])
                    ->formatStateUsing(fn (StorySubmissionStatus $state): string => $state->label()),
                TextColumn::make('created_at')
                    ->label('Примено')
                    ->dateTime('d.m.Y H:i')
                    ->sortable(),
            ])
            ->defaultSort('created_at', 'desc')
            ->filters([
                SelectFilter::make('status')
                    ->label('Статус')
                    ->options(fn () => collect(StorySubmissionStatus::cases())->mapWithKeys(fn ($s) => [$s->value => $s->label()])),
            ])
            ->recordActions([
                EditAction::make(),
            ])
            ->toolbarActions([]);
    }
}
