<?php

namespace App\Filament\Resources\StorySubmissions\Pages;

use App\Filament\Resources\StorySubmissions\StorySubmissionResource;
use Filament\Resources\Pages\ListRecords;

class ListStorySubmissions extends ListRecords
{
    protected static string $resource = StorySubmissionResource::class;

    protected function getHeaderActions(): array
    {
        return [];
    }
}
