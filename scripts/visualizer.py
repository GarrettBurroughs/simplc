import re
import graphviz
import sys


def parse_ast_tree(text_output):
    lines = text_output.split('\n')
    
    # UPDATED REGEX:
    # 1. ^(\s*)       -> Capture Indent
    # 2. (.*?)        -> Capture Content (Non-greedy, stops before the @)
    # 3. \s+@\s+      -> Look for the " @ " separator
    # 4. (\d+):(\d+)  -> Capture Row/Col
    # 5. .*$          -> Allow (and ignore) any trailing text like " ="
    pattern = re.compile(r"^(\s*)(.*?)\s+@\s+(\d+):(\d+).*$")

    root = None
    stack = [] 

    for line in lines:
        if not line.strip():
            continue

        match = pattern.match(line)
        
        if match:
            # Case 1: Standard node with location data
            indent_str, content, row, col = match.groups()
        else:
            # Case 2: Fallback for nodes that might miss location data (rare, but safe)
            # Just split by indentation
            fallback_match = re.match(r"^(\s*)(.*)$", line)
            if not fallback_match: continue
            indent_str, content = fallback_match.groups()
            row, col = "?", "?"

        # Normalize Indentation (2 spaces = 1 level)
        current_indent = len(indent_str)
        
        node = {
            "label": content.strip(),
            "children": [],
            "row": row,
            "col": col
        }

        if current_indent == 0:
            root = node
            stack = [(0, node)]
        else:
            while stack and stack[-1][0] >= current_indent:
                stack.pop()
            
            if stack:
                parent = stack[-1][1]
                parent["children"].append(node)
                stack.append((current_indent, node))
            else:
                # Handle implied roots or broken indentation
                if root is None:
                    root = node
                    stack = [(current_indent, node)]

    return root

def visualize(node, dot=None, parent_id=None, counter=[0]):
    if dot is None:
        dot = graphviz.Digraph(format='png')
        dot.attr(rankdir='TB', nodesep='0.4', ranksep='0.6')

    if not node:
        return dot

    # Unique ID for Graphviz
    node_id = f"n{counter[0]}"
    counter[0] += 1

    # Format the Label using HTML-like syntax for Graphviz
    # Bolds the Node Name, makes location smaller
    label_text = node['label']
    
    # Escape special HTML characters in label (like <, >) to prevent rendering errors
    label_text = label_text.replace("<", "&lt;").replace(">", "&gt;")
    
    loc_text = f"Line {node['row']}:{node['col']}"
    full_label = f'<<B>{label_text}</B><BR/><FONT POINT-SIZE="10" COLOR="gray30">{loc_text}</FONT>>'

    # Styling
    dot.node(node_id, full_label, shape='box', style='rounded,filled', fillcolor='lavender')

    if parent_id:
        dot.edge(parent_id, node_id)

    # Recurse
    for child in node['children']:
        visualize(child, dot, node_id, counter)

    return dot

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python script.py <ast_output_file>")
        sys.exit(1)

    try:
        with open(sys.argv[1], 'r') as f:
            raw_ast = f.read()
            
        print("Parsing AST...")
        parsed_data = parse_ast_tree(raw_ast)
        
        if parsed_data:
            print("Generating Graph...")
            graph = visualize(parsed_data)
            output_filename = "compiler_ast"
            graph.render(output_filename, view=True)
            print(f"Done! Saved to {output_filename}.png")
        else:
            print("Error: Could not parse any nodes from input file.")
            
    except FileNotFoundError:
        print(f"Error: File '{sys.argv[1]}' not found.")
