import rss from "@astrojs/rss";
import { SITE_TITLE, SITE_DESCRIPTION } from "../config";
import { getCollection } from "astro:content";

export async function get(context) {
  const games = await getCollection("game");
  return rss({
    title: SITE_TITLE,
    description: SITE_DESCRIPTION,
    site: import.meta.env.SITE,
    items: games.map((game) => ({
      title: game.data.title,
      releaseDate: game.data.releaseDate,
      description: game.data.description,
      link: `/games/${game.slug}/`,
    })),
  });
}
