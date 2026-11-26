<?php

namespace App\Http\Requests\Admin;

use Illuminate\Contracts\Validation\ValidationRule;
use Illuminate\Foundation\Http\FormRequest;

class StoreDonationNeedRequest extends FormRequest
{
    public function authorize(): bool
    {
        return true;
    }

    /**
     * @return array<string, ValidationRule|array<mixed>|string>
     */
    public function rules(): array
    {
        return [
            'title' => ['required', 'string', 'max:255'],
            'description' => ['required', 'string'],
            'sticky_content' => ['nullable', 'string'],
            'needed_amount' => ['required', 'integer', 'min:1', 'max:4294967295'],
            'image' => [$this->imageRequirement(), 'image', 'max:10240'],
        ];
    }

    /**
     * @return array<string, string>
     */
    public function messages(): array
    {
        return [
            'title.required' => 'Насловот е задолжителен.',
            'title.max' => 'Насловот може да содржи најмногу 255 знаци.',
            'description.required' => 'Описот е задолжителен.',
            'needed_amount.required' => 'Потребниот износ е задолжителен.',
            'needed_amount.integer' => 'Потребниот износ мора да биде цел број.',
            'needed_amount.min' => 'Потребниот износ мора да биде најмалку 1 МКД.',
            'needed_amount.max' => 'Потребниот износ е преголем.',
            'image.required' => 'Сликата е задолжителна.',
            'image.image' => 'Датотеката мора да биде слика.',
            'image.max' => 'Сликата може да биде најмногу 10 MB.',
        ];
    }

    protected function imageRequirement(): string
    {
        return 'required';
    }
}
