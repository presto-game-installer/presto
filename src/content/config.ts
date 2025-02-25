import { z, defineCollection } from "astro:content";

const gameSchema = z.object({
    title: z.string(),
    description: z.string(),
    releaseDate: z.coerce.date(),
    lastUpdatedDate: z.coerce.date(),
    homepageLink: z.string().optional(),
    githubLink: z.string().url(),
    version: z.string(),
    badge: z.string().optional(),
    supportedPlatforms: z.array(z.string()).refine(items => new Set(items).size === items.length, {
        message: 'supported platforms must be unique',
    }).optional(),
    tags: z.array(z.string()).refine(items => new Set(items).size === items.length, {
        message: 'tags must be unique',
    }).optional(),
    gameData: z.object({
        downloadPath: z.string().url().optional(),
        downloadFile: z.string().optional(),
    }).optional(),
    gamePlatforms: z.object({
        linux: z.object({
            downloadFile: z.string().optional(),
            gameExecutable: z.string().optional(),  
            gameWorkingDirAppend: z.string().optional(),
            dataInstallToGameDir: z.boolean().optional(),
            dataInstallToHomeDir: z.boolean().optional(),
            dataInstallPath: z.string().optional(),
        }).optional(),
        windows: z.object({
            downloadFile: z.string().optional(),
            gameExecutable: z.string().optional(),
            gameWorkingDirAppend: z.string().optional(),
            dataInstallToGameDir: z.boolean().optional(),
            dataInstallToHomeDir: z.boolean().optional(),
            dataInstallPath: z.string().optional(),
        }).optional(),
        macos: z.object({
            downloadFile: z.string().optional(),
            gameExecutable: z.string().optional(),
            gameWorkingDirAppend: z.string().optional(),
            dataInstallToGameDir: z.boolean().optional(),
            dataInstallToHomeDir: z.boolean().optional(),
            dataInstallPath: z.string().optional(),
            usesDMG: z.boolean().optional(),
        }).optional(),
    })
});

export type gameSchema = z.infer<typeof gameSchema>;

const gameCollection = defineCollection({ schema: gameSchema });

export const collections = {
    'game': gameCollection
};