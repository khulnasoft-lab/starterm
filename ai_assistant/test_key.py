def test_api_key():
    import os
    from dotenv import load_dotenv
    
    load_dotenv()
    key = os.getenv('OPENAI_API_KEY')
    
    if not key:
        print("API key is not set")
        return False
    if not key.startswith('sk-'):
        print("API key format is incorrect")
        return False
    print(f"API key found with length: {len(key)}")
    return True

test_api_key()

