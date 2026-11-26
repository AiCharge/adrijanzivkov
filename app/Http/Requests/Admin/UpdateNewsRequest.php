<?php

namespace App\Http\Requests\Admin;

class UpdateNewsRequest extends StoreNewsRequest
{
    protected function imageRequirement(): string
    {
        return 'nullable';
    }
}
