export interface NewsInterface {
    id: number;
    title: string;
    slug: string;
    short_description: string;
    main_image: string;
    main_image_url: string | null;
    content?: string;
    created_at: string;
}
