# sift
**sift** is a wrapper around fzf whose search list only includes local git repos in selected directories.
    - by default, the only directory it will search on is `C:\Users\<username>\Projects\`

## Dependencies
- [sift](https://github.com/junegunn/fzf)- for the fuzzy findng
- [sed](https://www.gnu.org/software/sed/) - for truncating `\.git\` from the output

## Usage
```
sift
```
just call sift and it'll open up the usual fzf interface but with only local git repos in selected directories.

## Extra
- 
- [sift](https://github.com/junegunn/fzf)
- 
