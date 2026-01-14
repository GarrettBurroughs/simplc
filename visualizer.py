import re
import graphviz
import sys

class ASTParser:
    def __init__(self, text):
        # Tokenize: Match words, numbers, strings, or special symbols like { } ( ) [ ] , :
        self.tokens = re.findall(r'\"[^\"]*\"|\w+|[{}()\[\],:]', text)
        self.pos = 0

    def peek(self):
        return self.tokens[self.pos] if self.pos < len(self.tokens) else None

    def consume(self, expected=None):
        token = self.tokens[self.pos]
        self.pos += 1
        return token

    def parse(self):
        token = self.peek()
        if token == "ASTNode":
            return self.parse_ast_node()
        elif token == "Some":
            self.consume("Some")
            self.consume("(")
            res = self.parse()
            self.consume(")")
            return res
        elif token == "None":
            self.consume("None")
            return None
        elif token == "[":
            return self.parse_list()
        elif token == '"' or token[0] == '"':
            return self.consume().strip('"')
        elif token.isdigit():
            return int(self.consume())
        else:
            # Handle Node Types like Program(...) or Function(...)
            node_type = self.consume()
            if self.peek() == "(":
                self.consume("(")
                args = []
                while self.peek() != ")":
                    args.append(self.parse())
                    if self.peek() == ",": self.consume(",")
                self.consume(")")
                return {"type": node_type, "args": args}
            return node_type

    def parse_ast_node(self):
        self.consume("ASTNode")
        self.consume("{")
        data = {}
        while self.peek() != "}":
            key = self.consume()
            self.consume(":")
            val = self.parse()
            data[key] = val
            if self.peek() == ",": self.consume(",")
        self.consume("}")
        return data

    def parse_list(self):
        self.consume("[")
        items = []
        while self.peek() != "]":
            items.append(self.parse())
            if self.peek() == ",": self.consume(",")
        self.consume("]")
        return items

def visualize(data, dot=None, parent=None, counter=[0]):
    if dot is None:
        dot = graphviz.Digraph(format='png')
        dot.attr(rankdir='TB', nodesep='0.4', ranksep='0.6')

    if not isinstance(data, dict) or "node" not in data:
        return dot

    # Unique ID for this node
    node_id = f"n{counter[0]}"
    counter[0] += 1

    # Extract Label
    content = data["node"]
    if isinstance(content, dict):
        label = content["type"]
        children = content["args"]
    else:
        label = str(content)
        children = []

    # Add Row/Col info to label
    full_label = f"<<B>{label}</B><BR/><FONT POINT-SIZE='10'>Line {data.get('row')}:{data.get('column')}</FONT>>"
    
    dot.node(node_id, full_label, shape='box', style='rounded,filled', fillcolor='lavender')

    if parent:
        dot.edge(parent, node_id)

    # Recurse through children
    for child in children:
        if isinstance(child, list):
            for item in child:
                visualize(item, dot, node_id, counter)
        else:
            visualize(child, dot, node_id, counter)

    return dot

f = open(sys.argv[1])

raw_ast = f.read()

parser = ASTParser(raw_ast)
parsed_data = parser.parse()
graph = visualize(parsed_data)
graph.render("compiler_ast", view=True)
