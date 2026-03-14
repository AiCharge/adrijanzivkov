<?php

namespace App\Filament\Resources\StorySubmissions\Schemas;

use App\Enums\StorySubmissionStatus;
use Filament\Forms;
use Filament\Schemas\Schema;

class StorySubmissionForm
{
    public static function configure(Schema $schema): Schema
    {
        return $schema
            ->components([
                Forms\Components\TextInput::make('name')
                    ->label('Име и презиме')
                    ->disabled(),
                Forms\Components\TextInput::make('phone')
                    ->label('Телефонски број')
                    ->disabled(),
                Forms\Components\Textarea::make('message')
                    ->label('Приказна')
                    ->rows(6)
                    ->disabled()
                    ->columnSpanFull(),
                Forms\Components\Select::make('status')
                    ->label('Статус')
                    ->options(StorySubmissionStatus::class)
                    ->required(),
                Forms\Components\Textarea::make('notes')
                    ->label('Белешки')
                    ->rows(4)
                    ->columnSpanFull(),
            ]);
    }
}
