import sys
sys.path.append('./scrapper/lib/python3.12/site-packages')


# from scrapper import Attachment, AttachmentType, Email

import rustimport.import_hook
from lib import Attachment, AttachmentType, Email


def main():
    attachment = Attachment("attachment.txt", AttachmentType.File)
    print(attachment)
    email = Email("This is the subject", "This is the body", [attachment])
    print(email)
    email.send("Example@gmail.com")


if __name__ == "__main__":
    main()