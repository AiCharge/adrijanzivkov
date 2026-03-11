<?php

namespace App\Filament\Resources\DonationNeedResource\Pages;

use App\Filament\Resources\DonationNeedResource;
use Filament\Actions;
use Filament\Resources\Pages\EditRecord;

class EditDonationNeed extends EditRecord
{
    protected static string $resource = DonationNeedResource::class;

    protected function getHeaderActions(): array
    {
        return [
            Actions\DeleteAction::make(),
        ];
    }
}
