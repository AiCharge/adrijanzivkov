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
use Filament\Tables\Table;

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
                    ->fileAttachmentsDisk('public')
                    ->fileAttachmentsVisibility('public')
                    ->required()
                    ->columnSpanFull(),
                Forms\Components\RichEditor::make('sticky_content')
                    ->label('Лепливa содржина (десна страна)')
                    ->fileAttachmentsDisk('public')
                    ->fileAttachmentsVisibility('public')
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
                    ->disk('public')
                    ->visibility('public')
                    ->directory('donation-needs'),
            ]);
    }

    public static function table(Table $table): Table
    {
        return $table
            ->columns([
                Tables\Columns\ImageColumn::make('image')
                    ->label('Слика')
                    ->disk('public'),
                Tables\Columns\TextColumn::make('title')
                    ->label('Наслов')
                    ->searchable()
                    ->sortable(),
                Tables\Columns\TextColumn::make('needed_amount')
                    ->label('Потребен износ')
                    ->money('MKD')
                    ->sortable(),
                Tables\Columns\TextColumn::make('created_at')
                    ->label('Креиран')
                    ->dateTime()
                    ->sortable(),
            ])
            ->filters([
                //
            ])
            ->recordActions([
                Actions\EditAction::make(),
            ])
            ->bulkActions([
                Actions\BulkActionGroup::make([
                    Actions\DeleteBulkAction::make(),
                ]),
            ]);
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
