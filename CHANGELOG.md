# 0.1.0 - 15/03/2021

## Legal

*  Added license.
*  Added license notice to files.

## Docs

*  Added README.md.
*  Added man pages.
*  Added CHANGELOG.md.

## Core

* Writing command runner.

# 0.1.1 - 15/03/2021

## Clean

*  Removing unused files.

## Fix

*  Fixing security breach. Fixing username getting (previously via $USER, that made faking possible) with syscall and /etc/passwd parsing.

## Scripts

*  Added uninstall rule to makefile.

# 0.1.2 - 16/03/2021

## Fix

*  Securising password system. Use of pam instead of parsing /etc/shadow.
*  Securising get_username. Using libc function getpwuid() instead of reading /etc/passwd.
