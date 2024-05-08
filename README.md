# ![RustyCloudServices](assets/branding/RustyCloudServices-Banner.png)
[![License](https://img.shields.io/badge/license-Apache-orange.svg)](https://github.com/RustyCloudServices/RustyCloud/blob/main/LICENSE)
![Created](https://img.shields.io/github/created-at/RustyCloudServices/RustyCloud?color=orange
)
[![Activity](https://img.shields.io/github/commit-activity/m/RustyCloudServices/RustyCloud?color=orange
)](https://github.com/RustyCloudServices/RustyCloud/graphs/contributors)

## A bit of a different take at Minecraft Clouds.

> [!CAUTION]
> This is in very early development and shouldn't be used around corrosive materials :)

## Our Design Goals

- **Fast**: Prioritize minimal memory usage and fast execution, with parallel processing when possible.
- **Versatile**: Offer best possible support for modern minecraft implementations.
- **Simple**: Ensure minimal configuration requirements for a seamless setup.
- **Reliable**: Designed to minimize errors within the cloud.

## Features
(✅ = Done, ⚙️ = currently working on, ❌ = not started)

- console & commands ⚙️
- start minecraft servers ([spigot](https://getbukkit.org/download/spigot)
  , [bukkit](https://getbukkit.org/download/craftbukkit) and [paper](https://papermc.io) based forks) ⚙️
- start proxy servers ([bungeecord](https://www.spigotmc.org/wiki/bungeecord/)
  , [waterfall](https://github.com/PaperMC/Waterfall), [velocity](https://github.com/PaperMC/Velocity)) ⚙️
- custom minecraft server implementation support ❌
- modify jvm flags for each group ❌
- print formatted server error to console ❌ 
- service templates ❌
- developer api (rust & java) ❌
- support for most sql databases ❌
- multi root / clustered ❌

NOTE: As stated, this cloud is in early development, and this feature list isn't final. We're open to suggestions.