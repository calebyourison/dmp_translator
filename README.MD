# DMP Serial 3 Event Translator
---
A very simple CLI program designed to translate each section of a Serial 3 message into its descriptive parts.

See releases for a Windows standalone executable.

**Please reference the official DMP documentation for specifics on their Serial 3 format.  This project is not suitable for processing live events. Please reach out to your DMP representative for guidance with regard to alarm receivers and live events.**

## Example Serial 3 events:
 ---
- Fire Alarm: Za\037\t “FI\z 001“OFFICE SMOKE DET\ee"RI\

- Open (Disarm) By User: Zq\059\t “OP\u 00001“LINDA SANCHEZ \a 001“OFFICE \

- Door Access By User: Zj\045\t “DA\v 001\u 00001“WILLIAM SMITH \

### Notes:
---
This is still very much a work in progress. At present, this program will only translate a single event and display the output in the console.

The Serial 3 message format has a few nuances to its structure that allow it to contain more information than some of the other alarm reporting formats. As a result, some of the less common events may not be translated properlly and further tweaks may be necessary.  Submissions for improvement are welcome.

## DISCLAIMER
**This project is in no way affiliated with DMP or any of its products.  Any usage of their products is subject to their terms of use.**

**This project is designed for educational and utilitarian use only. DO NOT RELY ON THIS PROJECT TO PROCESS LIVE ALARM EVENTS. Questions in this area should be directed to your DMP representative**

**Serial 3 events may contain confidential information such as user names and system component descriptions.  Be mindful of this when translating events to ensure that you do not violate anyone's privacy. This project takes no responsibility for misuse of privileged information.  If in doubt, consult proper legal counsel to ensure you are adhering to all applicable laws.**

