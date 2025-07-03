import os
from dotenv import load_dotenv
from openai import OpenAI

class AIAssistant:
    def __init__(self):
        load_dotenv()
        self.client = OpenAI(api_key=os.getenv('OPENAI_API_KEY'))
        self.model = "gpt-3.5-turbo"

    def get_response(self, prompt):
        try:
            response = self.client.chat.completions.create(
                model=self.model,
                messages=[
                    {"role": "system", "content": "You are a helpful AI assistant."},
                    {"role": "user", "content": prompt}
                ],
                max_tokens=150
            )
            return response.choices[0].message.content
        except Exception as e:
            return "Error: " + str(e)

def main():
    assistant = AIAssistant()
    print("Welcome to AI Assistant!")
    print("Type 'quit' to exit.")

    while True:
        user_input = input("\nYou: ")
        if user_input.lower() == 'quit':
            print("Goodbye!")
            break

        response = assistant.get_response(user_input)
        print("\nAI Assistant: " + response)

if __name__ == "__main__":
    main()
