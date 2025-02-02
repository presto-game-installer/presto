# Game Markdown Metadata Structure

In the `src/content/game` directory, each game has a markdown file that contains the game's metadata.

The aim is to make it as easy as possible to add a new game with the options being configurable enough to meet the needs of most games.

**Note:** The game's display image should be in the `/public/games` directory. The name of the file should be the slug of the game's title (spaces replaced with dashes, no special characters, all lowercase). Files should be in .png format.

- The game description goes in the body of the file.

- The front matter is the metadata for the game.

It is structured as follows:

```
---
title: "The Game's Title" (String)
description: "A Short Description of the Game" (String)
releaseDate: "Release Date of the Game [MM/DD/YYYY]" (String)
homepageLink: "a link to the game's homepage" (String)
githubLink: "a link to the game's github repo, if it exists" (String)
tags: "Tags for filtering games" (Array of Strings)
version: "Game Version" (String)
lastUpdatedDate: "Date game was last updated [MM/DD/YYYY]" (String)
gameData: {
    "downloadPath": "a link to where the game's files are hosted" (String)
    "downloadFile": "the name of the game's data file to download" (String)
}
supportedPlatforms: "Supported Platforms for this game [linux native, linux proton, windows, macos]" (Array of Strings)
gamePlatforms: {
    [Currently Supported Platforms: "linux", "windows", "macos"]
    "platform": {
        "downloadFile": "a link to the game's download file for the platform", (String)
        "gameExecutable": "the name of the game's executable file" (String)
        "gameWorkingDirAppend": "a path to be appended to the game's path before the executable" (String)
        "usesDMG": "true/false if the game's download file is a DMG file" (Boolean)
        "dataInstallToGameDir": "true/false if the game's data files should be installed to the game's directory" (Boolean)
        "dataInstallToHomeDir": "true/false if the game's data files should be installed to the user's home directory; NOTE that the dataInstallPath is appended to the home directory, if this is true" (Boolean)
        "dataInstallPath": "the path to install the game's data files to" (String)
    }
}
```

Note that the data options are platform specific, but the game data itself should be universal for all platforms.