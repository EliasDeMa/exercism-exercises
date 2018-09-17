def reverse(text):
    if len(text) == 0:
        return ""
    else:
        return text[-1] + reverse(text[:-1])
