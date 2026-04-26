<?php

namespace App\Filament\Resources\DonationNeedResource\Pages;

use App\Filament\Resources\DonationNeedResource;
use App\Models\DonationNeed;
use Filament\Actions;
use Filament\Resources\Pages\EditRecord;

class EditDonationNeed extends EditRecord
{
    protected static string $resource = DonationNeedResource::class;

    protected function getHeaderActions(): array
    {
        return [
            Actions\Action::make('archive')
                ->label('Архивирај')
                ->icon('heroicon-o-archive-box')
                ->color('warning')
                ->requiresConfirmation()
                ->modalHeading('Архивирај донациска потреба')
                ->modalDescription('Дали сте сигурни дека сакате да ја архивирате оваа донациска потреба? Таа нема да биде видлива на сајтот.')
                ->modalSubmitActionLabel('Архивирај')
                ->action(function (DonationNeed $record): void {
                    $record->update(['archived_at' => now()]);
                    $this->redirect(DonationNeedResource::getUrl('index'));
                })
                ->visible(fn (DonationNeed $record): bool => $record->archived_at === null),
            Actions\Action::make('unarchive')
                ->label('Врати')
                ->icon('heroicon-o-arrow-uturn-left')
                ->color('success')
                ->requiresConfirmation()
                ->modalHeading('Врати донациска потреба')
                ->modalDescription('Дали сте сигурни дека сакате да ја вратите оваа донациска потреба? Таа ќе биде повторно видлива на сајтот.')
                ->modalSubmitActionLabel('Врати')
                ->action(function (DonationNeed $record): void {
                    $record->update(['archived_at' => null]);
                    $this->redirect(DonationNeedResource::getUrl('index'));
                })
                ->visible(fn (DonationNeed $record): bool => $record->archived_at !== null),
        ];
    }
}
