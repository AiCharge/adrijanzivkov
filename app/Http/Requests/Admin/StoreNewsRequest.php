<?php

namespace App\Http\Requests\Admin;

use Illuminate\Contracts\Validation\ValidationRule;
use Illuminate\Foundation\Http\FormRequest;

class StoreNewsRequest extends FormRequest
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
            'short_description' => ['required', 'string', 'max:500'],
            'main_image' => [$this->imageRequirement(), 'image', 'max:10240'],
            'content' => ['required', 'string'],
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
            'short_description.required' => 'Краткиот опис е задолжителен.',
            'short_description.max' => 'Краткиот опис може да содржи најмногу 500 знаци.',
            'main_image.required' => 'Главната слика е задолжителна.',
            'main_image.image' => 'Датотеката мора да биде слика.',
            'main_image.max' => 'Сликата може да биде најмногу 10 MB.',
            'content.required' => 'Содржината е задолжителна.',
        ];
    }

    protected function imageRequirement(): string
    {
        return 'required';
    }
}
