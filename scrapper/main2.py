import sys
sys.path.append('./rustebookscrapper/scrapper/lib/python3.12/site-packages')

from scrapper import Attachment, AttachmentType, Email

def main():
    attachment = Attachment("attachment.txt", AttachmentType.File)
    print(attachment)
    email = Email("This is the subject", "This is the body", [attachment])
    print(email)
    email.send("Example@google.com")

 
if __name__ == "__main__":
    main()