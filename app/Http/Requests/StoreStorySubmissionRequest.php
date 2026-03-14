<?php

namespace App\Http\Requests;

use Illuminate\Foundation\Http\FormRequest;

class StoreStorySubmissionRequest extends FormRequest
{
    public function authorize(): bool
    {
        return true;
    }

    public function rules(): array
    {
        return [
            'name' => ['required', 'string', 'max:255'],
            'phone' => ['required', 'string', 'max:50'],
            'message' => ['required', 'string', 'max:5000'],
            'agreed' => ['accepted'],
        ];
    }

    public function messages(): array
    {
        return [
            'name.required' => 'Името е задолжително.',
            'name.max' => 'Името може да содржи најмногу 255 знаци.',
            'phone.required' => 'Телефонскиот број е задолжителен.',
            'phone.max' => 'Телефонскиот број е предолг.',
            'message.required' => 'Пораката е задолжителна.',
            'message.max' => 'Пораката може да содржи најмногу 5000 знаци.',
            'agreed.accepted' => 'Мора да се согласите со Условите на користење и Политиката за приватност.',
        ];
    }
}
