import { type Edge, type Node, Position } from '@xyflow/svelte';
import { type Writable } from 'svelte/store';
import dagre from '@dagrejs/dagre';
import { EngineWrapper } from '../../grafiek_wasm/pkg';

export class App {
	nodes: Writable<Node[]>;
	edges: Writable<Edge[]>;
	wrapper: EngineWrapper;

	constructor(wrapper: EngineWrapper, nodes: Writable<Node[]>, edges: Writable<Edge[]>) {

		const loadedNodes: Node[] = wrapper.list_nodes().map((node) => ({
			id: `${node.id}`,
			type: node.ty,
			data: { label: node.label, ty: node.ty, engine: wrapper, id: node.id },
			position: { x: 0, y: 0 },
			targetPosition: Position.Left,
			sourcePosition: Position.Right
		}));

		const loadedEdges: Edge[] = wrapper.list_edges().map((edge) => ({
			id: `${edge.source_node_id}-${edge.sync_node_id}`,
			target: `${edge.source_node_id}`,
			source: `${edge.sync_node_id}`
		}));


		// TODO: only run this on the condition that there is no layout info
		layout_graph(loadedNodes, loadedEdges);

		nodes.set(loadedNodes);
		edges.set(loadedEdges);

		this.wrapper = wrapper;
		this.nodes = nodes;
		this.edges = edges;
	}
}

//TODO: get actual node layout width and height
const nodeWidth = 256;
const nodeHeight = 256;

export const layout_graph = (nodes: Node[], edges: Edge[]) => {
	const dagreGraph = new dagre.graphlib.Graph();
	dagreGraph.setDefaultEdgeLabel(() => ({}));
	dagreGraph.setGraph({ rankdir: 'LR' });

	nodes.forEach((node) => {
		dagreGraph.setNode(node.id, { width: nodeWidth, height: nodeHeight });
	});

	edges.forEach((edge) => {
		dagreGraph.setEdge(edge.source, edge.target);
	});

	dagre.layout(dagreGraph);

	nodes.forEach((node) => {
		const nodeWithPosition = dagreGraph.node(node.id);

		node.position = {
			x: nodeWithPosition.x - nodeWidth / 2,
			y: nodeWithPosition.y - nodeHeight / 2
		};
	});
};
