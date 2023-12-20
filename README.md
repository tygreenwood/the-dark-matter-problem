# The Dark Matter Problem

## Table of Contents

- [Important Info for Using this Document!](#important-info-for-using-this-document)
- [Setup](#setup)
  - [Installing and Running the Project](#installing-and-running-the-project)
  - [Useful VSCode Extensions](#useful-vscode-extensions)
  - [VSCode Configuration](#vscode-configuration)
- [Git Good](#git-good)
  - [What is Git?](#what-is-git)
  - [Getting/Sending Updates](#gettingsending-updates)
    - [Get current project version](#get-current-project-version)
    - [Upload your changes to the World Wide Web](#upload-your-changes-to-the-world-wide-web)
  - [Branches](#branches)
    - [Switching to a new branch](#switching-to-a-new-branch)
    - [Creating a new branch](#creating-a-new-branch)
    - [Simple branch management](#simple-branch-management)
- [The Command Line](#the-command-line)
  - [Navigating the Command Line](#navigating-the-command-line)
  - [AHHH! VIM!](#ahhh-vim)
- [Working with the Project](#working-with-the-project)
  - [Programming File Help](#programming-file-help)
    - [To update existing image (background / sprite animation / etc.)](#to-update-existing-image-background--sprite-animation--etc)

## Important Info for Using this Document!

Any commands in which carrots `<>` are used, the carrots are meant to denote a word that should be replaced. You should not include the carrots in the actual command unless otherwise specified.

Ex:
```
git branch <branch>

// This should actually be
git branch my-branch-name
```

## Setup

### Installing and Running the Project

1. Install [Rust](https://www.rust-lang.org/tools/install)
1. Install [VSCode](https://code.visualstudio.com/download)
1. Clone the repository from GitHub
    <ol type="a">
      <li>If you haven't setup an SSH key with GitHub, follow <a href="https://docs.github.com/en/authentication/connecting-to-github-with-ssh/adding-a-new-ssh-key-to-your-github-account">these instructions</a></li>
      <li>Click the green "Code" button on the main project page, click the SSH tab in the popup, and copy the segment shown</li>
      <li>In your terminal, navigate to where you want the project folder to live, see <a href="#navigating-the-command-line">Navigating the Command Line</a> for help</li>
      <li>Last, execute the following command:</li>
    </ol>

    ```
    // Right click in the terminal to paste
    git clone <paste-segment-from-github>
    ```
1. Type `cargo run` in the command line to run the project

### Useful VSCode Extensions

Search extensions by going to the extensions tab on the left menu in VSCode, or by pressing (Ctrl+Shift+X) in VSCode. All links below will take you to the browser marketplace for these extensions, but you can find them in VSCode by searching for them in the extensions tab.

- [GitLens](https://marketplace.visualstudio.com/items?itemName=eamodio.gitlens)
  - This extensions provides a helpful Git interface and useful IDE features for Git.
- [crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates)
  - For developers, this simplifies crate management by showing whether or not a dependency our project has is out of date.
- [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)
  - Provides syntax highlighting for `.toml` files, only used for Cargo.toml in our project, but makes editing project configurations easier.
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
  - The most important extension for this project, this provides code completion, syntax highlighting, and formatting for Rust.
- [Rust Syntax](https://marketplace.visualstudio.com/items?itemName=dustypomerleau.rust-syntax)
  - Improved syntax highlighting for Rust.
- [Code Spell Checker](https://marketplace.visualstudio.com/items?itemName=streetsidesoftware.code-spell-checker)
  - Please. Let us not have spelling mistakes in our codebase. This will not flag conglomerated words used for variable and function names. It's very cool. I have added a `cspell.json` file we can use to suppress unwanted errors from this.
- [Prettier TOML](https://marketplace.visualstudio.com/items?itemName=bodil.prettier-toml)
  - This uses *Prettier* to format `.toml` files.

### VSCode Configuration

I find that VSCode doesn't automatically configure my formatters for this, so I recommend adding the following to your user `settings.json` file. You can open this file by first opening the command palette (ctrl+shift+p). Then search for `Preferences: Open User Settings (JSON)`.

Add the following configurations to the JSON object. If you have more configurations to add after this, make sure to put a comma after the last curly bracket. The corresponding VSCode extensions need to be installed for this to work.

```
    "editor.formatOnSave": true,
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer"
    },
    "[toml]": {
        "editor.defaultFormatter": "bodil.prettier-toml"
    }
```

## Git Good

Welcome to our git command tutorial! As we progress through this project, our usage of git is going to
get very complex! So here is our handy guide to what git is and how to use it.\

Before jumping into git, here's a few tips.

Git has a help command in case you forget a command or need clarification! Use `git help` to get a list.
You can also say `git help <command/concept>` for more details about a specific aspect.

A helpful VSCode extension for git is *GitLens*, which provides additional windows and information.

### What is Git?

Git is what we will be using to control different project versions! Think of Minecraft, where they
have 1.20.0, 1.20.1, et cetera. Our branch called `main` is where our primary version of the project
will live!

### Getting/Sending Updates

#### Get current project version

`git pull` (or `git pull origin main` if upstream branch is not configured) will get changes
from GitHub, specifically from the `main` branch. This will merge changes from the remote `main`
into whatever branch (see [Branches](#branches)) you are currently on, so make sure your changes
are committed first, and that you are on the branch you want to merge into!

The rest of this **Get current project version** section is for more advanced git use.

During your first `git pull`, git may ask you to specify how to reconcile divergent branches. For this project, we want to use the default option, which is merge. To set that in your local config, run the following:
```
// Essentially this sets the "rebase" strategy to false, which means we will use traditional merging
git config pull.rebase false
```

If you get merge conflicts, VSCode should highlight what files have issues,
specifically mark where in the file, and have options on how to fix these. After they are all fixed, you will need to go through the full commit process (including the `git add <File(s)>` step). When doing
the `git commit` step after merge conflicts, I typically avoid using the `-m` flag (short for message) and the corresponding message in quotes afterwards.

If you are trying to look at a branch other than main, I would use the `git fetch` command. That is all you
need to type, and then see the [Branches](#branches) section for how to switch to that branch. This will fetch all the changes from GitHub without bringing those changes into your working directory. This means that new branches will be fetched, and existing branches will be automatically updated IF you don't have any local changes to those branches. If you do have local changes, git fetch will not update those branches. Use `git pull` or `git merge` to update those branches.

For more on `git merge`, see [this article from Atlassian](https://www.atlassian.com/git/tutorials/using-branches/git-merge). However, this still doesn't fully cover merges, as it is one of the most complicated aspects of git.

If you want to merge that branch directly into whatever branch you are currently on (typically you would do this if you are already on the branch you are pulling and have local changes that aren't in GitHub), you would use `git pull origin <branch>` where `<branch>` is the name of the branch.

#### Upload your changes to the World Wide Web

At any point, `git status` will pull up a list of changes and show you if they've been added or not, as well as provide a few additional helpful commands if needed.

1. First, `git add`

   This command is used to stage your new changes for committing. Normally, you should use `git add .` to add all the files you've added/modified. THIS ONLY WORKS IF YOUR TERMINAL IS AT THE ROOT OF THE PROJECT.
   If your terminal is at the root of the project, it should look like this:
   ```
   // The $ symbol may be one of many different symbols depending on the terminal you use
   C:\<Path all the way to project>\the-dark-matter-problem $
   ```
   If a different path is shown, see [Navigating the Command Line](#navigating-the-command-line).

   You can also add individual files if you don't want to commit all your changes. Use `git add <path-to-file>` for each file you want to add.

1. Then, `git commit`

   This command takes your staged changes from `git add` and commits them. I typically use `git commit -m "<message here>"`, as just using `git commit` will open VIM to prompt for a message. Leaving a blank message will abort the commit. If you prefer to use that method, or accidentally don't use the `-m` flag, see [AHHH! VIM!](#ahhh-vim). Trust me, you will not escape without help.

1. Finally, `git push`
   This command functions just like [`git pull`](#get-current-project-version), but in reverse. Use `git push origin <branch>` to push your branch and its changes to GitHub. On its own, `git push` should push your main branch to the remote main branch. If that doesn't work, then `git push origin main` should get the job done.

### Branches

We don't just have one branch we're working on! For more complex changes, we might not want to just
push code straight into our primary version, what if it breaks?! We would want to make our changes in
a separate branch in order to avoid this.

<img src="https://upload.wikimedia.org/wikipedia/commons/a/a3/Politica_branch.jpg"
     alt="Git Branch Example" />

#### Switching to a new branch

```
// To switch what branch you are on, use the following command:
git checkout <branch>
```

#### Creating a new branch

When you create a new branch, that new branch will start as an exact copy of whatever branch you are on

```
// To create a new branch WITHOUT switching to it, you can use the checkout command to then switch
git branch <branch>

// To create a new branch AND switch to it
git checkout -b <branch>
/The -b flag indicates that the branch you are checking out will be a new branch
```

#### Simple branch management

```
// To see a list of branches
git branch

// To rename your current branch
git branch -m <new-name>

// To "safe" delete a branch
git branch -d <branch>
// That command will fail if the branch you are trying to delete has unmerged changes

// To force delete a branch
git branch -D <branch>
// This will delete the branch even if it has unmerged changes, be careful
```

## The Command Line

### Navigating the Command Line

Depending on the Operating System and Shell/Terminal you use, the commands will vastly differ. I will try to outline commands that should work for most situations here. I guarantee they work in both Powershell and Linux environments.

```
// This command takes you to your computer's root directory
cd ~
// For most people that should look like the following, with the $ being one of many potential symbols
C:\Users\<UserName> $

// This command takes you into a subfolder of whatever folder you are currently in
cd <folder-name>
// If you need to see your current folders, use the following (lowercase L below)
ls

// This command takes you up a folder level
cd ..
// So if you were here
C:\Users\<UserName>\Documents\Project
// You would now be here
C:\Users\<UserName>\Documents

// The previous command can be chained together
cd ../../..
// This will take you up three folders, so if you were here
C:\Users\<UserName>\Documents\Projects\the-dark-matter-problem\assets\sprites\platforms
// You would now be here
C:\Users\<UserName>\Documents\Projects\the-dark-matter-problem
// Which is where you would need to be for "git add ." to work!
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

### Programming File Help

#### To update existing image (background / sprite animation / etc.)

1. Drag your new image from your file explorer to the assets folder
1. Look for configs.rs in the specific file you are updating, eg mushroom or player
1. Replace the file path for the associated file you are replacing:
   ```
   // This is the old file name
   pub const SPACE_PLANET_ANIMATION_PATH: &str = "sprites/background/space/SpacePlanetAnimation.png";

   // This is the new file name, only edit what is in quotes
   pub const SPACE_PLANET_ANIMATION_PATH: &str = "sprites/background/moon/CrashingMoonAnimation.png";
   ```
1. Then, replace the file name with your new file and save
1. Do git shenanigans (see [Upload your changes to the World Wide Web](#upload-your-changes-to-the-world-wide-web))
