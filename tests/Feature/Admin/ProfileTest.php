<?php

use App\Models\User;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Illuminate\Support\Facades\Hash;

uses(RefreshDatabase::class);

beforeEach(function () {
    $this->user = User::factory()->create();
});

it('updates the name and email', function () {
    $this->actingAs($this->user)
        ->put(route('admin.profile.update'), [
            'name' => 'Нов Админ',
            'email' => 'new@example.com',
        ])
        ->assertRedirect(route('admin.profile.edit'))
        ->assertSessionHas('success');

    expect($this->user->fresh())
        ->name->toBe('Нов Админ')
        ->email->toBe('new@example.com');
});

it('keeps the password when none is given', function () {
    $this->actingAs($this->user)->put(route('admin.profile.update'), [
        'name' => $this->user->name,
        'email' => $this->user->email,
    ]);

    expect(Hash::check('password', $this->user->fresh()->password))->toBeTrue();
});

it('changes the password when the current password is correct', function () {
    $this->actingAs($this->user)
        ->put(route('admin.profile.update'), [
            'name' => $this->user->name,
            'email' => $this->user->email,
            'current_password' => 'password',
            'password' => 'new-secret-password',
            'password_confirmation' => 'new-secret-password',
        ])
        ->assertSessionHasNoErrors();

    expect(Hash::check('new-secret-password', $this->user->fresh()->password))->toBeTrue();
});

it('refuses a password change without the correct current password', function (?string $currentPassword) {
    $this->actingAs($this->user)
        ->put(route('admin.profile.update'), [
            'name' => $this->user->name,
            'email' => $this->user->email,
            'current_password' => $currentPassword,
            'password' => 'new-secret-password',
            'password_confirmation' => 'new-secret-password',
        ])
        ->assertSessionHasErrors('current_password');

    expect(Hash::check('password', $this->user->fresh()->password))->toBeTrue();
})->with([
    'missing' => null,
    'wrong' => 'not-my-password',
]);

it('requires the new password to be confirmed', function () {
    $this->actingAs($this->user)
        ->put(route('admin.profile.update'), [
            'name' => $this->user->name,
            'email' => $this->user->email,
            'current_password' => 'password',
            'password' => 'new-secret-password',
            'password_confirmation' => 'something-else',
        ])
        ->assertSessionHasErrors('password');
});

it('rejects an email that belongs to another user', function () {
    $otherUser = User::factory()->create();

    $this->actingAs($this->user)
        ->put(route('admin.profile.update'), [
            'name' => $this->user->name,
            'email' => $otherUser->email,
        ])
        ->assertSessionHasErrors('email');
});
