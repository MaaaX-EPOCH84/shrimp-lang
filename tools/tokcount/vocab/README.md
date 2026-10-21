# Frozen vocabularies

These files never change. A count made with them is reproducible for as
long as the repository exists, whatever happens to their publishers.

| File | Origin | Source file and SHA-256 | Licence |
|---|---|---|---|
| `o200k_base.tiktoken` | OpenAI, distributed with tiktoken 0.14.0 | https://openaipublic.blob.core.windows.net/encodings/o200k_base.tiktoken — `446a9538cb6c348e3516120d7c08b09f57c36495e2acfffe59a5bf8b0cfb1a2d` | MIT (tiktoken) |
| `cl100k_base.tiktoken` | OpenAI, distributed with tiktoken 0.14.0 | https://openaipublic.blob.core.windows.net/encodings/cl100k_base.tiktoken — `223921b76ee99bde995b7ff738513eef100fb51d18c93597a113bcffe865b2a7` | MIT (tiktoken) |
| `qwen3.merges` | Alibaba, Qwen/Qwen3-8B | https://huggingface.co/Qwen/Qwen3-8B/resolve/main/tokenizer.json — `aeb13307a71acd8fe81861d94ad54ab689df773318809eed3cbe794b4492dae4` | Apache-2.0 |
| `deepseek_v3.merges` | DeepSeek, deepseek-ai/DeepSeek-V3 | https://huggingface.co/deepseek-ai/DeepSeek-V3/resolve/main/tokenizer.json — `621ac2e32d0dba658404412318818aaa8ce8cda492e59830109d8da6b517fb41` | MIT |

The two `.tiktoken` files are copied as published: one `base64(token)
rank` per line. The two `.merges` files are produced from the published
`tokenizer.json` by `tokcount convert-hf`: one `base64(left)
base64(right)` per line, in merge order, bytes instead of the GPT-2
printable mapping. Both conversions are replayable from the source files
above.

The split patterns and the NFC flag that go with each vocabulary are in
`src/vocab.rs`, copied verbatim from tiktoken's `openai_public.py` and
from each `tokenizer.json`.
