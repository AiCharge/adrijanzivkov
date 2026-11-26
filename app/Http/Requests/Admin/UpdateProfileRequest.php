<?php

namespace App\Http\Requests\Admin;

use Illuminate\Contracts\Validation\ValidationRule;
use Illuminate\Foundation\Http\FormRequest;
use Illuminate\Validation\Rule;
use Illuminate\Validation\Rules\Password;

class UpdateProfileRequest extends FormRequest
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
            'name' => ['required', 'string', 'max:255'],
            'email' => ['required', 'string', 'email', 'max:255', Rule::unique('users')->ignore($this->user()->id)],
            'password' => ['nullable', 'string', Password::defaults(), 'confirmed'],
            'current_password' => ['required_with:password', 'nullable', 'current_password'],
        ];
    }

    /**
     * @return array<string, string>
     */
    public function messages(): array
    {
        return [
            'name.required' => 'Името е задолжително.',
            'name.max' => 'Името може да содржи најмногу 255 знаци.',
            'email.required' => 'Е-поштата е задолжителна.',
            'email.email' => 'Внесете валидна е-пошта.',
            'email.max' => 'Е-поштата може да содржи најмногу 255 знаци.',
            'email.unique' => 'Оваа е-пошта веќе се користи.',
            'password.confirmed' => 'Лозинките не се совпаѓаат.',
            'password.min' => 'Лозинката мора да содржи најмалку :min знаци.',
            'current_password.required_with' => 'Внесете ја тековната лозинка за да ја промените лозинката.',
            'current_password.current_password' => 'Тековната лозинка не е точна.',
        ];
    }
}
