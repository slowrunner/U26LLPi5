# Additional Notes 

### Tutorial 3 

[Step3: Translate, with a real LLM](https://github.com/URML-MARS/URML/blob/main/docs/tutorials/03-natural-language-to-urml.md#step-3-translate-with-a-real-llm)

Before attempting to install the OpenAI LLM Bridge, must be in URML/ folder:
```
cd /home/ubuntu/U26LLPi5/systests/urml/URML

now command will work:

pip install -e "reference/llm-bridge[openai]"
```

To configure Ollama with qwen2.5:0.5b:

source set_openai_ollama.sh
```
export OPENAI_BASE_URL: http://localhost:11434/v1/
export OPENAI_API_KEY: ollama qwen2.5:0.5b 
```

To configure remote Ollama (Mac ROS2Jazzy VM serving qwen2.5:3b)
source set_openai_ollama_remote.sh
```
export OPENAI_BASE_URL: http://10.0.0.xx:11434/v1/
export OPENAI_API_KEY: ollama qwen2.5:3b
```
