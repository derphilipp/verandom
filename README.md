# Verandom
## Overview

Verandom is a tool designed to bring random playback functionality to audio devices lacking this feature.
By renaming audio files with a random three-digit prefix, Verandom effectively shuffles your music collection,
allowing you to enjoy a randomized playlist on devices that do not support shuffle mode.

Perfect for users looking to add a touch of unpredictability to their listening experience without the need for specialized hardware or software.

## Features

Cross-Platform Compatibility: Works seamlessly on Linux, macOS and Windows.
Wide Range of Audio Formats: Supports popular audio formats, including `.mp3, .wav, .m4a, and .aac`.
User-Friendly: Simple graphical interface for selecting your music directory.
Unique Randomization: Ensures that each run produces a different order, using a unique three-digit prefix for every file.

## Installation

Just download a released version of Verandom

## Usage

Usage
To use Verandom, simply run the executable.
On the first run, a dialog will appear prompting you to select the directory containing your audio files.
Verandom will then process all compatible audio files within the directory, renaming them with a random three-digit prefix to achieve a randomized order.

## What does it do?

### Before Randomization
The original filenames might look something like this, assuming they are named in a sequential or non-random manner:

```
001 - Your Favorite Song.mp3
002 - Another Great Track.wav
003 - Awesome Beat.m4a
004 - Chill Vibes.aac
005 - Party Starter.mp3
```

Only files that have three leading digits are randomized!

### After Randomization
After running through Verandom, the files could be renamed as follows, with the original three-digit prefix replaced by a new,
randomly generated three-digit number. Note that the new numbers are randomly chosen for the sake of this example;
actual results will vary each time you run Verandom:

```
723 - Your Favorite Song.mp3
146 - Another Great Track.wav
582 - Awesome Beat.m4a
309 - Chill Vibes.aac
867 - Party Starter.mp3
```

As your player will order the files by filename, you now have a new order they are played in. Enjoy!
