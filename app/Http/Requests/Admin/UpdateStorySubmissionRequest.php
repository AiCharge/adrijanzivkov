<?php

namespace App\Http\Requests\Admin;

use App\Enums\StorySubmissionStatus;
use Illuminate\Contracts\Validation\ValidationRule;
use Illuminate\Foundation\Http\FormRequest;
use Illuminate\Validation\Rule;

class UpdateStorySubmissionRequest extends FormRequest
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
            'status' => ['required', Rule::enum(StorySubmissionStatus::class)],
            'notes' => ['nullable', 'string', 'max:10000'],
        ];
    }

    /**
     * @return array<string, string>
     */
    public function messages(): array
    {
        return [
            'status.required' => 'Статусот е задолжителен.',
            'status.enum' => 'Избраниот статус не е валиден.',
            'notes.max' => 'Белешките може да содржат најмногу 10000 знаци.',
        ];
    }
}
