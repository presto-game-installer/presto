import { z, defineCollection } from "astro:content";
const gameSchema = z.object({
    title: z.string(),
    description: z.string(),
    releaseDate: z.coerce.date(),
    lastUpdatedDate: z.coerce.date(),
    linuxDownload: z.string(),
    windowsDownload: z.string(),
    macDownload: z.string(),
    dataDownloadPath: z.string(),
    dataDownloadFile: z.string(),
    romInstallPath: z.string(),
    romInstallDir: z.string(),
    version:  z.string(),
    heroImage: z.string().optional(),
    badge: z.string().optional(),
    tags: z.array(z.string()).refine(items => new Set(items).size === items.length, {
        message: 'tags must be unique',
    }).optional(),
});

export type gameSchema = z.infer<typeof gameSchema>;

const gameCollection = defineCollection({ schema: gameSchema });

export const collections = {
    'game': gameCollection
}