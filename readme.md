## gash!
throughout the day, i usually do the following
1. `vim ~/temp/[random_name].md`
2. throw down words, format debugger output, etc.
3. `:wq`
4. `cd ..`
originally, i called these documents "scratchpads". per [some article] commands should be breezy; easy to type. scratchpad is not, gash is.

## goal
- open a markdown file, automatically name it, and save it into a directory
- retrieve these files using fuzzy string searching
- have a configurable file where you can set custom flags, directories, and search critera

## interface idas
`gash` - open a markdown vim file in ~/gash  
`gash title` - open a markdown with `title` vim file in ~/gash  
`gash -s words` - search for files with `words` using fuzzy logic  

