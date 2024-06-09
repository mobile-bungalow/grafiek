import { type Edge, type Node, Position, type Connection } from '@xyflow/svelte';
import { type Writable } from 'svelte/store';
import dagre from '@dagrejs/dagre';
import { EngineWrapper } from '../../grafiek_wasm/pkg';
import type { CommonNodeData, CommonEdgeData } from './common';

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
		}));

		const loadedEdges: Edge[] = wrapper.list_edges().map((edge) => ({
			data: { id: edge.id },
			id: `${edge.source_node_id}-${edge.sync_node_id}`,
			source: `${edge.source_node_id}`,
			target: `${edge.sync_node_id}`
		}));

		// TODO: only run this on the condition that there is no layout info
		layout_graph(loadedNodes, loadedEdges);

		nodes.set(loadedNodes);
		edges.set(loadedEdges);

		this.wrapper = wrapper;
		this.nodes = nodes;
		this.edges = edges;
	}

	remove_weights(removals: { nodes: Node[]; edges: Edge[] }) {
		for (const node of removals.nodes) {
			const data = node.data as CommonNodeData;
			this.wrapper.remove_node(data.id);
		}

		for (const edge of removals.edges) {
			console.log(edge);
			const data = edge.data as CommonEdgeData;
			this.wrapper.remove_edge(data.id);
		}
		this.wrapper.render();
	}

	connect(con: Connection): Edge {
		// TODO: actual target indices
		// TODO: add edge ID to the new edge
		const id = this.wrapper.connect_nodes(Number(con.source), Number(con.target), 0, 0);
		this.wrapper.render();
		return {	
			data: { id },
			id: `${con.source}-${con.target}`,
			source: `${con.source}`,
			target: `${con.target}`
		};
	}


	test_add() {
		const id = this.wrapper.add_node('Grief');
		this.nodes.update((nodesMut) => {
			nodesMut.push({
				id: `${id}`,
				type: 'GrayScale',
				data: { label: 'Grief', ty: 'GrayScale', engine: this.wrapper, id },
				position: { x: 0, y: 0 },
				targetPosition: Position.Left,
				sourcePosition: Position.Right
			});
			return nodesMut;
		});
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
