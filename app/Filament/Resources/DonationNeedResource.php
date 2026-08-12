<?php

namespace App\Filament\Resources;

use App\Filament\Resources\DonationNeedResource\Pages;
use App\Models\DonationNeed;
use BackedEnum;
use Filament\Actions;
use Filament\Forms;
use Filament\Resources\Resource;
use Filament\Schemas\Schema;
use Filament\Tables;
use Filament\Tables\Filters\TernaryFilter;
use Filament\Tables\Table;
use Illuminate\Database\Eloquent\Builder;

class DonationNeedResource extends Resource
{
    protected static ?string $model = DonationNeed::class;

    protected static BackedEnum|string|null $navigationIcon = 'heroicon-o-heart';

    protected static ?string $navigationLabel = 'Донации';

    protected static ?string $modelLabel = 'Донациска потреба';

    protected static ?string $pluralModelLabel = 'Донациски потреби';

    public static function form(Schema $schema): Schema
    {
        return $schema
            ->components([
                Forms\Components\TextInput::make('title')
                    ->label('Наслов')
                    ->required()
                    ->maxLength(255),
                Forms\Components\RichEditor::make('description')
                    ->label('Опис')
                    ->required()
                    ->columnSpanFull(),
                Forms\Components\RichEditor::make('sticky_content')
                    ->label('Лепливa содржина (десна страна)')
                    ->columnSpanFull(),
                Forms\Components\TextInput::make('needed_amount')
                    ->label('Потребен износ (МКД)')
                    ->required()
                    ->numeric()
                    ->minValue(1),
                Forms\Components\FileUpload::make('image')
                    ->label('Слика')
                    ->image()
                    ->required()
                    ->directory('donation-needs'),
            ]);
    }

    public static function table(Table $table): Table
    {
        return $table
            ->modifyQueryUsing(fn (Builder $query) => $query->withoutGlobalScopes())
            ->columns([
                Tables\Columns\ImageColumn::make('image')
                    ->label('Слика'),
                Tables\Columns\TextColumn::make('title')
                    ->label('Наслов')
                    ->searchable()
                    ->sortable(),
                Tables\Columns\TextColumn::make('needed_amount')
                    ->label('Потребен износ')
                    ->money('MKD')
                    ->sortable(),
                Tables\Columns\IconColumn::make('archived_at')
                    ->label('Архивирано')
                    ->boolean()
                    ->trueIcon('heroicon-o-archive-box')
                    ->falseIcon('heroicon-o-check-circle')
                    ->trueColor('warning')
                    ->falseColor('success')
                    ->getStateUsing(fn (DonationNeed $record): bool => $record->archived_at !== null),
                Tables\Columns\TextColumn::make('created_at')
                    ->label('Креиран')
                    ->dateTime()
                    ->sortable(),
            ])
            ->filters([
                TernaryFilter::make('archived_at')
                    ->label('Архивирани')
                    ->nullable()
                    ->placeholder('Сите')
                    ->trueLabel('Само архивирани')
                    ->falseLabel('Само активни')
                    ->queries(
                        true: fn (Builder $query) => $query->whereNotNull('archived_at'),
                        false: fn (Builder $query) => $query->whereNull('archived_at'),
                        blank: fn (Builder $query) => $query,
                    ),
            ])
            ->recordActions([
                Actions\Action::make('archive')
                    ->label('Архивирај')
                    ->icon('heroicon-o-archive-box')
                    ->color('warning')
                    ->requiresConfirmation()
                    ->modalHeading('Архивирај донациска потреба')
                    ->modalDescription('Дали сте сигурни дека сакате да ја архивирате оваа донациска потреба? Таа нема да биде видлива на сајтот.')
                    ->modalSubmitActionLabel('Архивирај')
                    ->action(fn (DonationNeed $record) => $record->update(['archived_at' => now()]))
                    ->visible(fn (DonationNeed $record): bool => $record->archived_at === null),
                Actions\Action::make('unarchive')
                    ->label('Врати')
                    ->icon('heroicon-o-arrow-uturn-left')
                    ->color('success')
                    ->requiresConfirmation()
                    ->modalHeading('Врати донациска потреба')
                    ->modalDescription('Дали сте сигурни дека сакате да ја вратите оваа донациска потреба? Таа ќе биде повторно видлива на сајтот.')
                    ->modalSubmitActionLabel('Врати')
                    ->action(fn (DonationNeed $record) => $record->update(['archived_at' => null]))
                    ->visible(fn (DonationNeed $record): bool => $record->archived_at !== null),
                Actions\EditAction::make(),
            ])
            ->bulkActions([]);
    }

    public static function getRelations(): array
    {
        return [];
    }

    public static function getPages(): array
    {
        return [
            'index' => Pages\ListDonationNeeds::route('/'),
            'create' => Pages\CreateDonationNeed::route('/create'),
            'edit' => Pages\EditDonationNeed::route('/{record}/edit'),
        ];
    }
}
