<?php

namespace App\Filament\Resources\StorySubmissions\Pages;

use App\Filament\Resources\StorySubmissions\StorySubmissionResource;
use Filament\Resources\Pages\EditRecord;

class EditStorySubmission extends EditRecord
{
    protected static string $resource = StorySubmissionResource::class;

    protected function getHeaderActions(): array
    {
        return [];
    }
}
