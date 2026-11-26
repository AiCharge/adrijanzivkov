/**
 * The first letters of a person's first two names, for an avatar without a picture.
 */
export const initials = (name: string): string =>
    name
        .split(' ')
        .filter(Boolean)
        .slice(0, 2)
        .map((part) => part[0])
        .join('')
        .toUpperCase();
