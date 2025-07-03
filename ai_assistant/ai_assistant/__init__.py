# -*- coding: utf-8 -*-
import os
import logging
import sys
from dotenv import load_dotenv
from openai import OpenAI, OpenAIError
import openai
import httpx

# Function to verify API key format
def verify_env_file(path='.env'):
    import re
    try:
        with open(path, 'r') as f:
            content = f.read()
    
        # Check if OPENAI_API_KEY is present and properly formatted
        key_match = re.search(r'OPENAI_API_KEY\s*=\s*([^\n]+)', content)
        if not key_match:
            print("OPENAI_API_KEY not found in .env file")
            return False
            
        key_value = key_match.group(1).strip()
        if not key_value.startswith('sk-'):
            print("OPENAI_API_KEY format appears incorrect (should start with 'sk-')")
            return False
        
        print("OPENAI_API_KEY format appears correct")
        return True
    except Exception as e:
        print(f"Error verifying .env file: {str(e)}")
        return False

# Load environment variables
load_dotenv()

# Configure logging
logging.basicConfig(level=logging.INFO)

# System prompt instructions
SYSTEM_PROMPT = """
I'm an AI assistant with a set of instructions that guide my behavior, which include:

1. I operate within Warp, the AI terminal, to assist with software development tasks
2. I use specific tools like run_command, code, read_files, and grep
3. I follow strict rules about command execution, file handling, and code management
4. I'm constrained to work only within the terminal environment
5. I have specific guidelines about handling secrets and sensitive information
6. I aim to complete tasks exactly as requested - no more, no less

=== Agent Mode Instructions ===

1. General Purpose:
- I am Agent Mode, an AI Agent running within Starterm, the AI terminal
- My purpose is to assist with software development tasks in the terminal
- I must never assist with unsafe or malicious code/tasks

2. Available Tools:
- run_command: Executes shell commands
- code: Delegates coding tasks to specialized coding agent
- read_files: Reads contents of files
- grep: Performs text-based searches in files/directories

3. Command Execution Rules:
- Never use interactive or fullscreen commands
- Use non-paginated output versions when possible
- Only change directories when necessary
- Never reveal or consume secrets in plain-text
- Never edit files directly with terminal commands - use the code tool instead

4. File Handling:
• Use read_files tool for reading files, not terminal commands like cat/head/tail
• For large files (>5000 lines), request exactly 5000-line chunks at a time
• Always request entire files unless they would be truncated

5. Coding Guidelines:
• Use the code tool for all coding tasks
• Provide clear, well-defined task descriptions
• Don't include file contents in task arguments as they're passed implicitly

6. Task Completion:
• Do exactly what was requested - no more, no less
• Don't automatically execute follow-up actions without user confirmation
• Bias toward action for explicit user requests

7. Security Rules:
• Never assist with potentially unsafe or malicious code
• Never reveal or consume secrets in plain-text
• Handle security with utmost care

Please provide clear and specific instructions for the tasks you want me to perform.
I will assist you with software development tasks while ensuring safety and security.
"""

class AIAssistant:
    def __init__(self):
        # Initialize OpenAI client
        # Verify API key format
        if not verify_env_file():
            raise ValueError("Invalid or missing OPENAI_API_KEY in .env file")
            
        api_key = os.getenv('OPENAI_API_KEY')
        if not api_key:
            raise ValueError("OPENAI_API_KEY not found in environment variables.")
        http_client = httpx.Client(
            base_url="https://api.openai.com/v1",
            timeout=60.0
        )
        
        self.client = OpenAI(
            api_key=api_key,
            http_client=http_client
        )
        self.model = "gpt-3.5-turbo"

    def get_response(self, prompt):
        try:
            response = self.client.chat.completions.create(
                model=self.model,
                messages=[
                    {"role": "system", "content": SYSTEM_PROMPT},
                    {"role": "user", "content": prompt}
                ],
                max_tokens=150,
                temperature=0.7
            )
            return response.choices[0].message.content.strip()
        except OpenAIError as e:
            logging.error("OpenAI API error: %s", str(e))
            if "insufficient_quota" in str(e):
                return ("Error: OpenAI API quota has been exceeded. Please:\n"
                       "1. Check your API key is correct\n"
                       "2. Verify your billing status at https://platform.openai.com/account/billing\n"
                       "3. Add a payment method if you haven't already\n"
                       "4. Consider upgrading your API plan if needed")
            else:
                return f"OpenAI API Error: {str(e)}"
        except Exception as e:
            logging.exception("Unexpected error:")
            return f"Unexpected Error: {str(e)}"

def main():
    assistant = AIAssistant()
    print("Welcome to AI Assistant!")
    print("Type 'quit' to exit.")

    while True:
        try:
            print("\nYou: ", end="", flush=True)
            user_input = sys.stdin.readline().strip()
            if user_input.lower() == 'quit':
                print("Goodbye!")
                break
            if not user_input:
                print("Please enter a prompt.")
                continue

            response = assistant.get_response(user_input)
            print("\nAI Assistant:", response)

        except KeyboardInterrupt:
            print("\nInterrupted. Goodbye!")
            break
        except EOFError:
            print("\nEOF detected. Goodbye!")
            break
        except Exception as e:
            print(f"An error occurred: {str(e)}")
            break

if __name__ == "__main__":
    main()
