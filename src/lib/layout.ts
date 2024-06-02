import { type Edge, type Node } from '@xyflow/svelte';
import dagre from '@dagrejs/dagre';

//TODO: get actual node layout width and height
const nodeWidth = 172;
const nodeHeight = 36;

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
