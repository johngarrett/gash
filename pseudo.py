
if __name__ == '__main__':
    bash("cd GASH_DIR")
    bash("vim arg[1].md ?? {random_name}.md")
    follow_file()
    on file close {
        write {keywords, date, and other attributes}
            into file metadata
    }

def follow_file:
    every X seconds:
        - read through file
        - estimate a new title
        keywords.append(important_word)
