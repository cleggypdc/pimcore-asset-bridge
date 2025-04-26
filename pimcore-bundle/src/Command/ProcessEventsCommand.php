<?php

namespace AssetBridgeBundle\Command;

use Pimcore\Model\Asset;
use Symfony\Component\Console\Attribute\AsCommand;
use Symfony\Component\Console\Command\Command;
use Symfony\Component\Console\Input\InputInterface;
use Symfony\Component\Console\Output\OutputInterface;
use Symfony\Component\Console\Input\InputOption;
use Symfony\Component\Console\Style\SymfonyStyle;

#[AsCommand(
    name: 'asset-bridge:process-events',
    description: 'Processes filesystem change events into Pimcore assets',
)]
class ProcessEventsCommand extends Command
{
    protected function configure(): void
    {
        $this
            ->addOption('input', null, InputOption::VALUE_OPTIONAL, 'Path to input JSON file. If omitted, reads from STDIN');
    }

    protected function execute(InputInterface $input, OutputInterface $output): int
    {
        $io = new SymfonyStyle($input, $output);

        $jsonInput = $input->getOption('input');

        if ($jsonInput) {
            $content = file_get_contents($jsonInput);
        } else {
            $content = stream_get_contents(STDIN);
        }

        if (empty($content)) {
            $io->error('No input received.');
            return Command::FAILURE;
        }

        $events = json_decode($content, true);

        if (!is_array($events)) {
            $io->error('Invalid JSON input.');
            return Command::FAILURE;
        }

        foreach ($events as $event) {
            $this->handleEvent($event, $io);
        }

        $io->success('Processed ' . count($events) . ' events.');

        return Command::SUCCESS;
    }

    private function handleEvent(array $event, SymfonyStyle $io): void
    {
        $eventType = $event['event'] ?? null;
        $path = $event['path'] ?? null;

        if (!$eventType || !$path) {
            $io->warning('Skipping invalid event');
            return;
        }

        switch ($eventType) {
            case 'created':
                $this->handleCreate($path, $io);
                break;
            case 'modified':
                $this->handleModify($path, $io);
                break;
            case 'deleted':
                $this->handleDelete($path, $io);
                break;
            case 'renamed':
                $from = $event['from'] ?? null;
                $to = $event['to'] ?? null;
                if ($from && $to) {
                    $this->handleRename($from, $to, $io);
                }
                break;
            case 'moved_in':
                $this->handleCreate($path, $io);
                break;
            case 'moved_out':
                $this->handleDelete($path, $io);
                break;
            default:
                $io->warning('Unknown event type: ' . $eventType);
                break;
        }
    }

    private function handleCreate(string $path, SymfonyStyle $io): void
    {
        $io->text("Handle create for: $path");
        // TODO: Create Asset in Pimcore
    }

    private function handleModify(string $path, SymfonyStyle $io): void
    {
        $io->text("Handle modify for: $path");
        // TODO: Update Asset in Pimcore
    }

    private function handleDelete(string $path, SymfonyStyle $io): void
    {
        $io->text("Handle delete for: $path");
        // TODO: Delete Asset in Pimcore
    }

    private function handleRename(string $from, string $to, SymfonyStyle $io): void
    {
        $io->text("Handle rename from $from to $to");
        // TODO: Rename Asset in Pimcore
    }
}
