export interface DonationNeedInterface {
    id: number;
    title: string;
    description: string;
    image: string;
    image_url: string | null;
    needed_amount: number;
    created_at: string;
}
