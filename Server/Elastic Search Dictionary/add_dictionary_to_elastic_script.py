import dictionary as dict
import requests
from requests.auth import HTTPBasicAuth
import threading
import os

base_url = "https://localhost:9200/part_of_speech"
auth_username = 'elastic'
auth_password = os.getenv('ELASTIC_PASSWORD')

res = requests.delete(base_url, verify=False, auth=HTTPBasicAuth(auth_username, auth_password))
print(res.json())

requests.put(base_url, verify=False, auth=HTTPBasicAuth(auth_username, auth_password), json=
{
    "mappings": {
        "dynamic": "strict",
        "properties": {
            "word": {
                "type": "keyword"
            },
            "part_of_speech": {
                "type": "keyword"
            },
            "sub_part_of_speech": {
                "type": "keyword"
            }
        }
    }
}
             )


def post_data(category_array, part_of_speech, subcategory):
    starting_id = 0
    for category in dict.catergories:
        if category == category_array:
            break
        starting_id = starting_id + len(category)
    for word in category_array:
        starting_id = starting_id + 1
        url = base_url + "/_create/" + str(starting_id)
        request = requests.post(url, verify=False, auth=HTTPBasicAuth(auth_username, auth_password), json=
        {
            "word": word,
            "part_of_speech": part_of_speech,
            "sub_part_of_speech": subcategory
        })
        print("================================================================")
        print(request.status_code)
        print("----------------------------------------------------------------")
        print(request.json())
        print("================================================================")


simple_present = threading.Thread(target=post_data, args=(dict.simple_present, "verb", "simple_present"))
simple_present.start()

simple_past = threading.Thread(target=post_data, args=(dict.simple_past, "verb", "simple_past"))
simple_past.start()

past_participle = threading.Thread(target=post_data, args=(dict.past_participle, "verb", "past_participle"))
past_participle.start()

present_third_person = threading.Thread(target=post_data,
                                        args=(dict.present_third_person, "verb", "present_third_person"))
present_third_person.start()

present_participle = threading.Thread(target=post_data, args=(dict.present_participle, "verb", "present_participle"))
present_participle.start()

adjectives = threading.Thread(target=post_data, args=(dict.adjectives, "adjective", "adjective"))
adjectives.start()

determiners = threading.Thread(target=post_data, args=(dict.determiners, "determiner", "basic_determiner"))
determiners.start()

determiners_numbers = threading.Thread(target=post_data,
                                       args=(dict.determiners_numbers, "determiner", "determiner_number"))
determiners_numbers.start()

adverbs = threading.Thread(target=post_data, args=(dict.adverbs, "adverb", "adverb"))
adverbs.start()

personal_pronouns = threading.Thread(target=post_data, args=(dict.personal_pronouns, "pronoun", "personal_pronoun"))
personal_pronouns.start()

object_pronouns = threading.Thread(target=post_data, args=(dict.object_pronouns, "pronoun", "object_pronoun"))
object_pronouns.start()

possessive_pronouns = threading.Thread(target=post_data,
                                       args=(dict.possessive_pronouns, "pronoun", "possessive_pronoun"))
possessive_pronouns.start()

possessive_adjectives = threading.Thread(target=post_data,
                                         args=(dict.possessive_adjectives, "pronoun", "possessive_adjective"))
possessive_adjectives.start()

reflexive_pronouns = threading.Thread(target=post_data, args=(dict.reflexive_pronouns, "pronoun", "reflexive_pronoun"))
reflexive_pronouns.start()

indefinite_pronouns = threading.Thread(target=post_data,
                                       args=(dict.indefinite_pronouns, "pronoun", "indefinite_pronoun"))
indefinite_pronouns.start()

demonstrative_pronouns = threading.Thread(target=post_data,
                                          args=(dict.demonstrative_pronouns, "pronoun", "demonstrative_pronoun"))
demonstrative_pronouns.start()

interrogative_pronouns = threading.Thread(target=post_data,
                                          args=(dict.interrogative_pronouns, "pronoun", "interrogative_pronoun"))
interrogative_pronouns.start()

relative_pronouns = threading.Thread(target=post_data, args=(dict.relative_pronouns, "pronoun", "relative_pronoun"))
relative_pronouns.start()

archaic_pronouns = threading.Thread(target=post_data, args=(dict.archaic_pronouns, "pronoun", "archaic_pronoun"))
archaic_pronouns.start()

prepositions = threading.Thread(target=post_data, args=(dict.prepositions, "preposition", "preposition"))
prepositions.start()

coordinating_conjunctions = threading.Thread(target=post_data, args=(
    dict.coordinating_conjunctions, "conjunction", "coordinating_conjunction"))
coordinating_conjunctions.start()

correlative_conjunctions = threading.Thread(target=post_data,
                                            args=(dict.correlative_conjunctions, "conjunction", "correlative_conjunction"))
correlative_conjunctions.start()

subordinating_conjunctions = threading.Thread(target=post_data, args=(
    dict.subordinating_conjunctions, "conjunction", "subordinating_conjunction"))
subordinating_conjunctions.start()

interjections = threading.Thread(target=post_data, args=(dict.interjections, "interjection", "interjection"))
interjections.start()
