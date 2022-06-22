import { open } from '@tauri-apps/api/shell';
import {
  AppBar,
  Button,
  Card,
  CardContent,
  Grid,
  List,
  ListItem,
  ListItemText,
  Toolbar,
  Container,
  Typography,
  TextField,
  Pagination,
  IconButton,
  ButtonGroup,
  Tooltip,
} from '@mui/material';
import { useState, useEffect, useCallback } from 'react';
import { invoke, clipboard } from '@tauri-apps/api';
import { ToastContainer, toast, Slide } from 'react-toastify';
import { useForm, SubmitHandler } from 'react-hook-form';
import 'react-toastify/dist/ReactToastify.css';
import dayjs from 'dayjs';
import {
  ArrowDownward,
  ArrowUpward,
  CalendarMonth,
  FormatListBulleted,
  SortByAlpha,
} from '@mui/icons-material';
import { intersection } from 'lodash';

interface LinkInfo {
  title: string;
  desc: string;
  url: string;
  name: string;
  created_time: any;
  score: number;
}
interface OpenGraph {
  /// Represents the "og:title" OpenGraph meta tag.
  ///
  /// The title of your object as it should appear within
  /// the graph, e.g., "The Rock".
  title: string;
  /// Represents the "og:description" OpenGraph meta tag
  description: string;
  /// Represents the "og:url" OpenGraph meta tag
  url: string;
  /// Represents the "og:image" OpenGraph meta tag
  image: string;
  /// Represents the "og:type" OpenGraph meta tag
  ///
  /// The type of your object, e.g., "video.movie". Depending on the type
  /// you specify, other properties may also be required.
  object_type: string;
  /// Represents the "og:locale" OpenGraph meta tag
  locale: string;
}
interface Config {
  mode: SortMode;
  score: string[];
}
interface LinkListItemProps {
  link: LinkInfo;
  index: number;
}
type SortMode = 'normal' | 'date' | 'score';
const Home = () => {
  const [mode, setMode] = useState<SortMode>('normal');

  const [linkInfos, setLinkInfos] = useState<LinkInfo[]>([]);
  const { register, handleSubmit, reset } = useForm<LinkInfo>();
  const [page, setPage] = useState(0);
  const itemPerPage = 5;
  const pageCount = Math.ceil(linkInfos.length / itemPerPage);

  const createLink: SubmitHandler<LinkInfo> = (data) => {
    invoke('create_link', {
      ...data,
    }).then(() => {
      // Forced refreash view to ensure updated list.
      refreshInfo();
      toast('Link created!');
      reset();
    });
  };

  // Refresh all info.
  const refreshInfo = useCallback(async () => {
    const names = (await invoke('read_link_list')) as string[];

    const links = await Promise.all(
      names.map(async (val) => {
        const link = {
          ...(await invoke('read_link', { name: val })),
          name: val,
          score: 0,
        } as LinkInfo;
        return link;
      })
    ).catch((e) => {
      console.error(e);
      throw e;
    });

    const defaultConfig: Config = {
      mode: 'score',
      score: links.map((val) => val.name),
    };
    // Get config or initialize one.
    const initialConfig = await invoke<Config>('get_config').catch((e) => {
      console.log('failed to get config', e);
      // Fallback to create the default config
      return invoke<Config>('set_config', {
        config: defaultConfig,
      });
    });

    console.log('initialConfig:', initialConfig);
    // Merge List item into config score when mode is set to score.
    if (mode === 'score') {
      // merge config score with current linkList
      //
      // if a item is added/deleted from disk, it will disappeared on the list.
      let mergedScore = intersection(
        initialConfig.score,
        links.map((val) => val.name)
      );
      console.log('merged:', mergedScore);
      console.log('links', links);
      let mergedLinkInfos = mergedScore.map((val) => {
        const link = links.find((link) => {
          return link.name === val;
        });
        return link;
      }) as LinkInfo[];

      console.log('mergedLinkInfos:', mergedLinkInfos);
      setLinkInfos(mergedLinkInfos);
      return;
    }
    // Sort and push link infos
    setLinkInfos(
      (links as LinkInfo[])
        // eslint-disable-next-line array-callback-return
        .sort((a, b) => {
          switch (mode) {
            case 'normal':
              return a.title.localeCompare(b.title);
            case 'date':
              return (
                b.created_time.secs_since_epoch -
                a.created_time.secs_since_epoch
              );
            // Default to score
          }
        })
      // TODO
    );
  }, [mode]);

  useEffect(() => {
    refreshInfo();
  }, [refreshInfo]);

  const updateScore = (arr: LinkInfo[]) => {
    const updatedConfig: Config = {
      mode: 'score',
      score: arr.map((val) => val.name),
    };
    console.log('updatedConfig:', updatedConfig);
    invoke('set_config', {
      config: updatedConfig,
    })
      .then((_) => {
        console.log('successfully set config');
      })
      .catch((e) => {
        throw e;
      });
  };
  const LinkList = () => {
    const LinkListItem = ({ link, index }: LinkListItemProps) => {
      const [previewInfo, setPreviewInfo] = useState<OpenGraph>();
      useEffect(() => {
        invoke('generate_link_preview', {
          url: link.url.toString(),
        }).then((val) => setPreviewInfo(val as OpenGraph));
      }, [link.url]);

      return (
        <Tooltip
          arrow
          title={
            <>
              <Typography variant='body2'>
                {previewInfo?.title ??
                  'Preview may not available at the moment'}
              </Typography>
              <img
                loading='lazy'
                alt='preview'
                src={previewInfo?.image}
                width={250}></img>
            </>
          }>
          <ListItem
            dense
            key={index}
            sx={{
              display: 'flex',
              flexDirection: 'column',
              alignItems: 'flex-start',
            }}>
            <ListItemText
              primary={
                <Typography
                  sx={{
                    maxWidth: '25rem',
                  }}
                  paragraph>
                  {link.title}
                </Typography>
              }
              secondary={dayjs
                .unix(link.created_time.secs_since_epoch)
                .toDate()
                .toLocaleString()}></ListItemText>

            <Grid container>
              <Grid item container m='auto'>
                <Button
                  onClick={() => {
                    clipboard.writeText(link.url);
                  }}>
                  {'COPY'}
                </Button>
                <Button
                  onClick={() => {
                    open(link.url.toString());
                  }}>
                  OPEN
                </Button>
                <Button
                  onClick={() => {
                    invoke('delete_link', {
                      name: link.name,
                    });
                    refreshInfo();
                    toast('Link deleted!');
                  }}
                  color='error'>
                  DELETE
                </Button>

                <IconButton
                  color='primary'
                  disabled={index === 0 || mode !== 'score'}
                  onClick={() => {
                    let arr = Array.from(linkInfos);
                    [arr[index - 1], arr[index]] = [arr[index], arr[index - 1]];
                    console.log(arr);
                    setLinkInfos(arr);
                    updateScore(arr);
                  }}>
                  <ArrowUpward />
                </IconButton>
                <IconButton
                  color='error'
                  disabled={index === linkInfos.length - 1 || mode !== 'score'}
                  onClick={() => {
                    let arr = Array.from(linkInfos);
                    [arr[index], arr[index + 1]] = [arr[index + 1], arr[index]];
                    console.log(arr);
                    setLinkInfos(arr);
                    updateScore(arr);
                  }}>
                  <ArrowDownward />
                </IconButton>
              </Grid>
            </Grid>
          </ListItem>
        </Tooltip>
      );
    };
    return (
      <List>
        {linkInfos
          .map((val, idx) => {
            return (
              <div id={val.title} key={val.title}>
                <LinkListItem link={val} index={idx} />
              </div>
            );
          })
          .slice(itemPerPage * page, itemPerPage * (page + 1))}
      </List>
    );
  };

  return (
    <>
      <AppBar position='fixed'>
        <Toolbar>
          <Typography
            variant='h6'
            sx={{
              flexGrow: 1,
            }}>
            ARK Shelf
          </Typography>
        </Toolbar>
      </AppBar>
      <Toolbar />
      <Container
        sx={{
          mt: 2,
        }}>
        <Grid container spacing={8}>
          <Grid item xs={8}>
            <Card>
              <CardContent>
                <LinkList />

                <Pagination
                  count={pageCount === 0 ? 1 : pageCount}
                  page={page + 1}
                  onChange={(_, page) => {
                    // mui pages are started from 1, against to zero-based index array
                    setPage(page - 1);
                  }}
                  showFirstButton
                  showLastButton
                />
              </CardContent>
            </Card>
          </Grid>
          <Grid item xs={4}>
            <Grid item>
              <ButtonGroup variant='outlined'>
                <Tooltip key='alphabet' title={'Sorting By Alphabet'}>
                  <IconButton
                    onClick={() => {
                      setMode('normal');
                    }}>
                    <SortByAlpha />
                  </IconButton>
                </Tooltip>

                <Tooltip key='date' title={'Sorting By Date'}>
                  <IconButton
                    onClick={() => {
                      setMode('date');
                    }}>
                    <CalendarMonth />
                  </IconButton>
                </Tooltip>
                <Tooltip key='score' title={'Sorting By Score'}>
                  <IconButton
                    onClick={() => {
                      setMode('score');
                    }}>
                    <FormatListBulleted />
                  </IconButton>
                </Tooltip>
              </ButtonGroup>
            </Grid>
            <form onSubmit={handleSubmit(createLink)}>
              <TextField
                fullWidth
                label='url'
                margin='normal'
                {...register('url', { required: true })}></TextField>
              <TextField
                fullWidth
                label='title'
                margin='normal'
                {...register('title', { required: true })}></TextField>
              <TextField
                fullWidth
                label='description(optional)'
                margin='normal'
                {...register('desc', {
                  required: false,
                  value: '',
                })}></TextField>

              <Button type='submit'>Create</Button>
            </form>
          </Grid>
        </Grid>
      </Container>
      <ToastContainer
        position='bottom-right'
        autoClose={1000}
        hideProgressBar
        newestOnTop={false}
        closeOnClick
        rtl={false}
        pauseOnFocusLoss
        draggable
        pauseOnHover
        transition={Slide}
        theme='dark'
      />
    </>
  );
};
export default Home;
