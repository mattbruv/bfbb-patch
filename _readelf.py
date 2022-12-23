import pathlib
import sys
import json
from elftools.elf.elffile import ELFFile
from elftools.elf.relocation import RelocationSection


def parseElf(elf: ELFFile):
    parsed = []

    sectionIndex = 0

    for section in elf.iter_sections():
        if "debug" in section.name or "line" in section.name or "mwcats" in section.name:
            sectionIndex += 1 
            continue
        print(section.name, "->", section)
        print(section.data_alignment)
        print()
        sec = {}
        sec["index"] = sectionIndex
        sec["name"] = section.name
        sec["header"] = {}

        for thing in section.header:
            sec["header"][thing] = section.header[thing]
        
        # parse relocations
        if isinstance(section, RelocationSection):
            sec["relocations"] = []
            for reloc in section.iter_relocations():
                info = {}
                for thing in reloc.entry:
                    info[thing] = reloc.entry[thing]
                sec["relocations"].append(info)
        
        # handle symbols
        if section.name == ".symtab":
            sec["symbols"] = []
            for symbol in section.iter_symbols():
                sym = {}
                sym["name"] = symbol.name
                things = ["st_name", "st_value", "st_size"]
                for t in things:
                    sym[t] = symbol.entry[t]
                
                print(symbol.name, symbol.entry)
                sec["symbols"].append(sym)

        sec["data"] = str(section.data())

        parsed.append(sec)
        sectionIndex += 1


    return parsed

def readElf(path):

    p = pathlib.Path(path)
    file = open(p, "rb")
    elf = ELFFile(file)
    parsed = parseElf(elf)
    out = p.name.split('.')[0] + ".json"
    open(out, "w").write(json.dumps(parsed, indent=4))



if __name__ == "__main__":
    readElf(sys.argv[1])

