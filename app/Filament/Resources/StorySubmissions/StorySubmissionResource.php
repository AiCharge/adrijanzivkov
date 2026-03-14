<?php

namespace App\Filament\Resources\StorySubmissions;

use App\Filament\Resources\StorySubmissions\Pages\EditStorySubmission;
use App\Filament\Resources\StorySubmissions\Pages\ListStorySubmissions;
use App\Filament\Resources\StorySubmissions\Schemas\StorySubmissionForm;
use App\Filament\Resources\StorySubmissions\Tables\StorySubmissionsTable;
use App\Models\StorySubmission;
use BackedEnum;
use Filament\Resources\Resource;
use Filament\Schemas\Schema;
use Filament\Support\Icons\Heroicon;
use Filament\Tables\Table;

class StorySubmissionResource extends Resource
{
    protected static ?string $model = StorySubmission::class;

    protected static string|BackedEnum|null $navigationIcon = Heroicon::OutlinedInbox;

    protected static ?string $navigationLabel = 'Поднесени приказни';

    protected static ?string $modelLabel = 'Приказна';

    protected static ?string $pluralModelLabel = 'Поднесени приказни';

    public static function form(Schema $schema): Schema
    {
        return StorySubmissionForm::configure($schema);
    }

    public static function table(Table $table): Table
    {
        return StorySubmissionsTable::configure($table);
    }

    public static function getRelations(): array
    {
        return [];
    }

    public static function getPages(): array
    {
        return [
            'index' => ListStorySubmissions::route('/'),
            'edit' => EditStorySubmission::route('/{record}/edit'),
        ];
    }
}
