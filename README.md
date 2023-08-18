# rust_ls

A very basic implementation of ls in rust.

## RDD

1. Running `rust_ls` in a directory will list all files and directories in the `pwd`
2. Running `rust_ls` with a path to another directory (e.g., `rust_ls ../`) will have the same behavior as step 1, but in the given directory

## Thinking through the problem domain

A cursory glance at `man ls` shows that the existing flags fall into these categories (some example flags included):
- Filtering (`-A/-a`)
- Display Formatting (`-B`, `-C`, `-F`, `-G`, `-T`)
- Sorting (`-S`, `-r`, `-U`, `-c`)
- Behavioral (`-R`, `-H`, `-P`)

Looking at the behavioral category, there are two approaches that can be taken when walking the pwd -
1. Only walk the pwd
2. Walk the pwd and all subdirs

These can be modeled as strategies. Beyond that, `-H` and `-P` flags are exclusive to how we handle symlinks (they are also mutually exclusive). Symlink behavior can be applied to either strategy.

Sorting can be done at runtime, as the directory contents are iterated over. They can then be inserted into a vec in sorted order. A common `trait` with a single `insert` method could be used, then implemented by each sort method. Alternatively the name of the attribute by which the results should be sorted could be passed in.

Displaying could be handled by implementing `fmt::Display` multiple times on the struct that represents a file or directory, maybe.

Finally, filtering can be done when iterating through files on load, as a part of the behavioral strategy.

So ultimately, we get _something_ like this -

```rust
trait ListStrategy {
    fn load(&self, pwd) {}
}

enum SymLinkBehavior {
    Follow(&Path),
    Print(&Path),
}

impl SymLinkBehavior {
    fn do_it(&self) {}
}

enum Filter {
    IncludeDotDirs,
    ExcludeDotDotAndDot, // or something..
}

impl Filter {
    fn do_it(&self) {}
}

struct StandardList {
    SymLinkHandler: SymLinkBehavior,
    FilterHandler: Filter,
}

struct RecursiveList {
    SymLinkHandler: SymLinkBehavior,
    FilterHandler: Filter,
}

// impl ListStrategy for ... blocks excluded for brevity //

struct INode {
    // ... stuff here ... /
}

struct FileSystem {
    root: INode,
    strategy: ListStrategy,

    pub fn load(&self) {
        self.strategy.load();
    }
}
```
