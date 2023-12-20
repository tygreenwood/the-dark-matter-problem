# The Dark Matter Problem

## Table of Contents

- [Important Info for Using this Document!](#important-info-for-using-this-document)
- [Setup](#setup)
- [Git Good](#git-good)
- [Navigating the Command Line](#navigating-the-command-line)
- [Working with the Project](#working-with-the-project)

## Important Info for Using this Document!

Any commands in which carrots `<>` are used, the carrots are meant to denote a word that should be replaced. You should not include the carrots in the actual command unless otherwise specified.

Ex:
```
git branch <branch>

//This should actually be
git branch my-branch-name
```

## Setup

1. Install [Rust](https://www.rust-lang.org/tools/install)
1. Install [VSCode](https://code.visualstudio.com/download)
1. Type `cargo run` in the command line

## Git Good

Welcome to our git command tutorial! As we progress through this project, our usage of git is going to
get very complex! So here is our handy guide to what git is and how to use it.

### What is Git?

Git is what we will be using to control different project versions! Think of Minecraft, where they
have 1.20.0, 1.20.1, et cetera. Our branch called `main` is where our primary version of the project
will live!

### Getting/Sending Updates

#### Get current project version

`git pull` (or `git pull origin main` if upstream branch is not configured) will get changes
from GitHub, specifically from the `main` branch. This will merge changes from the remote `main`
into whatever branch (see [Branches](#branches)) you are currently on, so make sure your changes
are commited first, and that you are on the branch you want to merge into!

If you get merge conflicts, VSCode should highlight what files have issues,
specifically mark where in the file, and have options on how to fix these. After they are all fixed, you will need to go through the full commit process (including the `git add <File(s)>` step). When doing
the `git commit` step after merge conflicts, I typically avoid using the `-m` flag (short for message) and the corresponding message in quotes afterwards.

If you are trying to look at a branch other than main, I would use the `git fetch` command. That is all you
need to type, and then see the [Branches](#branches) section for how to switch to that branch. If you want to merge that branch directly into whatever branch you are currently on (typically you would do this if you are already on the branch you are pulling and have local changes that aren't in GitHub), you would use `git pull origin <branch>` where `<branch>` is the name of the branch.

#### Upload your changes to the World Wide Web

At any point, `git status` will pull up a list of changes and show you if they've been added or not, as well as provide a few additional helpful commands if needed.

1. First, `git add`

   This command is used to stage your new changes for committing. Normally, you should use `git add .` to add all the files you've added/modified. THIS ONLY WORKS IF YOUR TERMINAL IS AT THE ROOT OF THE PROJECT.
   If your terminal is at the root of the project, it should look like this:
   ```
   //The $ symbol may be one of many different symbols depending on the terminal you use
   C:\<Path all the way to project>\the-dark-matter-problem $
   ```
   If a different path is shown, see [Navigating the Command Line](#navigating-the-command-line).

   You can also add individual files if you don't want to commit all your changes. Use `git add <path-to-file>` for each file you want to add.

1. Then, `git commit`

   This command takes your staged changes from `git add` and commits them. I typically use `git commit -m "<message here>"`, as just using `git commit` will open VIM to prompt for a message. Leaving a blank message will abort the commit. If you prefer to use that method, or accidentally don't use the `-m` flag, see [AHHH! VIM!](#ahhh-vim). Trust me, you will not escape without help.

1. Finally, `git push`
   This command functions just like `git pull`, but in reverse. Use `git push origin <branch>` to push your branch and its changes to GitHub. On its own, `git push` should push your main branch to the remote main branch. If that doesn't work, then `git push origin main` should get the job done.

### Branches

We don't just have one branch we're working on! For more complex changes, we might not want to just
push code straight into our primary version, what if it breaks?! We would want to make our changes in
a separate branch in order to avoid this.

<img src="https://upload.wikimedia.org/wikipedia/commons/a/a3/Politica_branch.jpg"
     alt="Git Branch Example" />

## The Command Line

### Navigating the Command Line

Depending on the Operating System and Shell/Terminal you use, the commands will vastly differ. I will try to outline commands that should work for most situations here. I guarantee they work in both Powershell and Linux environments.

```
//This command takes you to your computer's root directory
cd ~
//For most people that should look like the following, with the $ being one of many potential symbols
C:\Users\<UserName> $

//This command takes you into a subfolder of whatever folder you are currently in
cd <folder-name>
//If you need to see your current folders, use the following (lowercase L below)
ls

//This command takes you up a folder level
cd ..
//So if you were here
C:\Users\<UserName>\Documents\Project
//You would now be here
C:\Users\<UserName>\Documents

//The previous command can be chained together
cd ../../..
//This will take you up three folders, so if you were here
C:\Users\<UserName>\Documents\Projects\the-dark-matter-problem\assets\sprites\platforms
//You would now be here
C:\Users\<UserName>\Documents\Projects\the-dark-matter-problem
//Which is where you would need to be for "git add ." to work!
```

### AHHH! VIM!
<img src="https://cdn.stackoverflow.co/images/jo7n4k8s/production/7a0bf96c6e3155ca56c74723cb0c0767517a4429-324x318.jpg?auto=format"
     alt="One does not simply exit VIM" />

In VIM, there's two basic modes. Edit and command (not official names).
By default, it should launch in command (hopefully).

To enter edit mode from command mode, press `i` (for insert). This will allow you to type into the window, such as you might need to do for a `git commit` message.

To enter command mode from edit mode, press `esc` (escape key). This will allow you to enter the magical combination of keys needed to exit vim.

To exit vim, there are a few potential commands. After entering each command, press enter to execute that action.

1. If no changes have been made, `:q` (colon, q for quit, and press enter) should quit VIM.
1. If changes have been made and you want to save them, such as a git commit message, `:wq` (colon, w for write, q for quit, and press enter) will save and quit VIM.
1. If changes have been made but you don't want to save them, or your panicking as you realize you are stuck in this hellscape of VIM, use `:q!` to force quit.

## Working with the Project

#### To update existing image (background / sprite animation / etc.)

1. Drag your new image from your file explorer to the assets folder
1. Look for configs.rs in the specific file you are updating, eg mushroom or player
1. Replace the file path for the associated file you are replacing:
   ```
   //This is the old file name
   pub const SPACE_PLANET_ANIMATION_PATH: &str = "sprites/background/space/SpacePlanetAnimation.png";

   //This is the new file name, only edit what is in quotes
   pub const SPACE_PLANET_ANIMATION_PATH: &str = "sprites/background/moon/CrashingMoonAnimation.png";
   ```
1. Then, replace the file name with your new file and save
1. Do git shenangins above
