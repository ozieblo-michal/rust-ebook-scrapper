import sys
sys.path.append('./rustebookscrapper/scrapper/lib/python3.12/site-packages')

import scrapper

def main():
    print(scrapper.sum_as_string(1, 2))

if __name__ == "__main__":
    main()
