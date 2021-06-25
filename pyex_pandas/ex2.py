import pandas as pd

FRAME_SIZE = 300
RATIO = 11.1

def is_fake_max_age(age, i):
    is_max = True
    for j in range(i + 1, min(i + 1 + FRAME_SIZE, len(age))):
        age_j = age[j]
        age_i = age[i]
        if age_j is not None and age_i is not None and age_j > age_i * RATIO:
            is_max = False
            break
    return age

tab = pd.read_csv("../confirmed-cases.csv")
age = tab["age"]
max_age_col = [is_fake_max_age(age, i) for i in range(len(age))]
print(len(max_age_col), max_age_col[10000])

