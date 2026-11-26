<?php

use App\Models\User;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Inertia\Testing\AssertableInertia as Assert;

uses(RefreshDatabase::class);

it('shows the login page to guests', function () {
    $this->get(route('admin.login'))
        ->assertSuccessful()
        ->assertInertia(fn (Assert $page) => $page->component('Admin/Auth/Login'));
});

it('redirects guests from admin pages to the login page', function (string $routeName) {
    $this->get(route($routeName))->assertRedirect(route('admin.login'));
})->with([
    'admin.index',
    'admin.donation-needs.index',
    'admin.news.index',
    'admin.story-submissions.index',
    'admin.profile.edit',
]);

it('logs users in with valid credentials', function () {
    $user = User::factory()->create();

    $this->post(route('admin.login.store'), [
        'email' => $user->email,
        'password' => 'password',
    ])->assertRedirect(route('admin.index'));

    $this->assertAuthenticatedAs($user);
});

it('redirects to the page the user wanted after logging in', function () {
    $user = User::factory()->create();

    $this->get(route('admin.news.index'));

    $this->post(route('admin.login.store'), [
        'email' => $user->email,
        'password' => 'password',
    ])->assertRedirect(route('admin.news.index'));
});

it('rejects an invalid password', function () {
    $user = User::factory()->create();

    $this->post(route('admin.login.store'), [
        'email' => $user->email,
        'password' => 'wrong-password',
    ])->assertSessionHasErrors('email');

    $this->assertGuest();
});

it('requires an email and password', function () {
    $this->post(route('admin.login.store'), [])
        ->assertSessionHasErrors(['email', 'password']);
});

it('locks the user out after too many failed attempts', function () {
    $user = User::factory()->create();

    foreach (range(1, 5) as $attempt) {
        $this->post(route('admin.login.store'), ['email' => $user->email, 'password' => 'wrong-password']);
    }

    $this->post(route('admin.login.store'), [
        'email' => $user->email,
        'password' => 'password',
    ])->assertSessionHasErrors(['email' => 'Премногу обиди за најава. Обидете се повторно за 60 секунди.']);

    $this->assertGuest();
});

it('redirects logged in users away from the login page', function () {
    $this->actingAs(User::factory()->create())
        ->get(route('admin.login'))
        ->assertRedirect(route('admin.index'));
});

it('redirects the admin root to donation needs', function () {
    $this->actingAs(User::factory()->create())
        ->get(route('admin.index'))
        ->assertRedirect('/admin/donation-needs');
});

it('logs users out', function () {
    $this->actingAs(User::factory()->create())
        ->post(route('admin.logout'))
        ->assertRedirect(route('admin.login'));

    $this->assertGuest();
});

it('shares the authenticated user with admin pages', function () {
    $user = User::factory()->create();

    $this->actingAs($user)
        ->get(route('admin.profile.edit'))
        ->assertInertia(fn (Assert $page) => $page
            ->component('Admin/Profile/Edit')
            ->where('auth.user.email', $user->email)
            ->missing('auth.user.password'));
});

it('renders admin pages with the admin root view and public pages with the site view', function () {
    $this->get(route('admin.login'))->assertViewIs('admin');

    $this->get(route('news.index'))->assertViewIs('app');
});
