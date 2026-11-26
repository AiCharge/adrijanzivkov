<?php

namespace App\Http\Requests\Admin;

class UpdateDonationNeedRequest extends StoreDonationNeedRequest
{
    protected function imageRequirement(): string
    {
        return 'nullable';
    }
}
