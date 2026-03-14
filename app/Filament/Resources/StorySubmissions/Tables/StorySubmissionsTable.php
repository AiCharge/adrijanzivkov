<?php

namespace App\Filament\Resources\StorySubmissions\Tables;

use App\Enums\StorySubmissionStatus;
use Filament\Actions\EditAction;
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
                    ->label('Име')
                    ->searchable()
                    ->sortable(),
                TextColumn::make('phone')
                    ->label('Телефон'),
                TextColumn::make('message')
                    ->label('Приказна')
                    ->limit(60)
                    ->tooltip(fn ($record) => $record->message),
                TextColumn::make('status')
                    ->label('Статус')
                    ->badge(),
                TextColumn::make('created_at')
                    ->label('Примено')
                    ->dateTime('d.m.Y H:i')
                    ->sortable(),
            ])
            ->defaultSort('created_at', 'desc')
            ->filters([
                SelectFilter::make('status')
                    ->label('Статус')
                    ->options(StorySubmissionStatus::class),
            ])
            ->recordActions([
                EditAction::make(),
            ])
            ->toolbarActions([]);
    }
}
