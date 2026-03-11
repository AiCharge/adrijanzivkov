export interface DonationNeedInterface {
    id: number;
    title: string;
    description: string;
    sticky_content: string | null;
    image: string;
    image_url: string | null;
    needed_amount: number;
    created_at: string;
}
